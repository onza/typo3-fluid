//! Downloads `fluid-lsp.tar.gz` from GitHub Releases — never uses `builtin/` at runtime.

use std::{env, fs};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{
    self as zed, DownloadedFileType, GithubReleaseOptions, LanguageServerId,
    LanguageServerInstallationStatus, Result, Worktree,
};

struct FluidExtension {
    cached_server_path: Option<String>,
}

impl FluidExtension {
    const SERVER_ID: &'static str = "fluid-lsp";
    const GITHUB_REPO: &'static str = "onza/typo3-fluid";
    const ASSET_NAME: &'static str = "fluid-lsp.tar.gz";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const HINT_MISSING_COMPANION: &'static str = "\
Fluid: live analysis (fluid:analyze) needs praetorius/fluid-companion on TYPO3 12/13. \
TYPO3 14+ ships analyze in Core — do not install Companion there. \
Install on 12/13: composer require --dev praetorius/fluid-companion. \
Disable: lsp.fluid-lsp.settings.show_companion_hint = false";

    fn version_dir() -> String {
        format!("fluid-lsp-{}", Self::VERSION)
    }

    fn relative_server_path() -> String {
        format!("{}/server.js", Self::version_dir())
    }

    fn absolute_path(relative: &str) -> Result<String> {
        let absolute = env::current_dir()
            .map_err(|e| format!("failed to resolve extension working directory: {e}"))?
            .join(relative);
        Ok(absolute.to_string_lossy().into_owned())
    }

    fn read_project_file(worktree: &Worktree, relative: &str) -> Option<String> {
        worktree.read_text_file(relative).ok().or_else(|| {
            std::fs::read_to_string(
                std::path::PathBuf::from(worktree.root_path()).join(relative),
            )
            .ok()
        })
    }

    fn composer_mentions(worktree: &Worktree, needles: &[&str]) -> bool {
        for file in ["composer.json", "composer.lock"] {
            if let Some(text) = Self::read_project_file(worktree, file) {
                if needles.iter().any(|n| text.contains(n)) {
                    return true;
                }
            }
        }
        false
    }

    fn has_typo3_context(worktree: &Worktree) -> bool {
        Self::composer_mentions(
            worktree,
            &[
                "typo3/cms-core",
                "typo3/cms-fluid",
                "typo3/cms-base-distribution",
                "typo3/cms-composer-installers",
            ],
        )
    }

    fn has_companion(worktree: &Worktree) -> bool {
        Self::composer_mentions(worktree, &["praetorius/fluid-companion"])
    }

    fn typo3_major(worktree: &Worktree) -> Option<u32> {
        for file in ["composer.lock", "composer.json"] {
            let Some(text) = Self::read_project_file(worktree, file) else {
                continue;
            };
            for needle in [r#""name": "typo3/cms-core""#, "typo3/cms-core"] {
                if let Some(idx) = text.find(needle) {
                    let window = &text[idx..text.len().min(idx + 200)];
                    if let Some(major) = Self::parse_major_near(window) {
                        return Some(major);
                    }
                }
            }
        }
        None
    }

    fn parse_major_near(text: &str) -> Option<u32> {
        let bytes = text.as_bytes();
        let mut i = 0;
        while i + 1 < bytes.len() {
            if bytes[i].is_ascii_digit() {
                let start = i;
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    i += 1;
                }
                if i < bytes.len() && bytes[i] == b'.' {
                    let major: u32 = text[start..i].parse().ok()?;
                    if (10..=20).contains(&major) {
                        return Some(major);
                    }
                }
            }
            i += 1;
        }
        None
    }

    fn setting_bool(worktree: &Worktree, key: &str, default: bool) -> bool {
        let Ok(lsp) = LspSettings::for_worktree(Self::SERVER_ID, worktree) else {
            return default;
        };
        for bag in [lsp.settings.as_ref(), lsp.initialization_options.as_ref()] {
            if let Some(value) = bag.and_then(|v| v.get(key)) {
                if let Some(flag) = value.as_bool() {
                    return flag;
                }
            }
        }
        default
    }

    fn companion_hint_if_needed(worktree: &Worktree) -> Option<String> {
        if !Self::has_typo3_context(worktree)
            || !Self::setting_bool(worktree, "show_companion_hint", true)
        {
            return None;
        }
        let major = Self::typo3_major(worktree)?;
        if (12..=13).contains(&major) && !Self::has_companion(worktree) {
            Some(Self::HINT_MISSING_COMPANION.into())
        } else {
            None
        }
    }

    fn download_server(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        if let Some(path) = &self.cached_server_path {
            if fs::metadata(path).is_ok_and(|m| m.is_file()) {
                return Ok(path.clone());
            }
        }

        let relative = Self::relative_server_path();
        if fs::metadata(&relative).is_ok_and(|m| m.is_file()) {
            let absolute = Self::absolute_path(&relative)?;
            self.cached_server_path = Some(absolute.clone());
            return Ok(absolute);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let tag = format!("v{}", Self::VERSION);
        let release = zed::github_release_by_tag_name(Self::GITHUB_REPO, &tag).or_else(|_| {
            zed::latest_github_release(
                Self::GITHUB_REPO,
                GithubReleaseOptions {
                    require_assets: true,
                    pre_release: false,
                },
            )
        })?;

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == Self::ASSET_NAME)
            .ok_or_else(|| {
                format!(
                    "GitHub release '{}' has no asset '{}'. Create it with: \
./tools/pack-fluid-lsp.sh && gh release upload {} dist/{} --clobber",
                    release.version,
                    Self::ASSET_NAME,
                    tag,
                    Self::ASSET_NAME
                )
            })?;

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

        let version_dir = Self::version_dir();
        zed::download_file(
            &asset.download_url,
            &version_dir,
            DownloadedFileType::GzipTar,
        )
        .map_err(|e| format!("failed to download {}: {e}", Self::ASSET_NAME))?;

        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let Some(name) = name.to_str() else {
                    continue;
                };
                if name.starts_with("fluid-lsp-") && name != version_dir {
                    let _ = fs::remove_dir_all(entry.path());
                }
            }
        }

        if !fs::metadata(&relative).is_ok_and(|m| m.is_file()) {
            return Err(format!(
                "downloaded Fluid helper missing at '{relative}' after extracting {}",
                Self::ASSET_NAME
            ));
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::None,
        );

        let absolute = Self::absolute_path(&relative)?;
        self.cached_server_path = Some(absolute.clone());
        Ok(absolute)
    }

    fn server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let server = self.download_server(language_server_id)?;
        let schema_dir = {
            let mut p = std::path::PathBuf::from(&server);
            p.pop();
            p.push("schemas");
            p.to_string_lossy().into_owned()
        };

        let live = Self::setting_bool(worktree, "live_template_analysis", true);
        let use_ddev = Self::setting_bool(worktree, "use_ddev", true);
        let generate_schema = Self::setting_bool(worktree, "generate_viewhelper_schema", true);

        let mut env_vars = vec![
            ("FLUID_SCHEMA_DIR".into(), schema_dir),
            ("FLUID_WORKTREE_ROOT".into(), worktree.root_path()),
            (
                "FLUID_LIVE_ANALYSIS".into(),
                if live { "1" } else { "0" }.into(),
            ),
            (
                "FLUID_USE_DDEV".into(),
                if use_ddev { "1" } else { "0" }.into(),
            ),
            (
                "FLUID_GENERATE_SCHEMA".into(),
                if generate_schema { "1" } else { "0" }.into(),
            ),
        ];
        if let Some(hint) = Self::companion_hint_if_needed(worktree) {
            env_vars.push(("FLUID_ANALYZE_HINT".into(), hint));
        }

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![server],
            env: env_vars,
        })
    }
}

impl zed::Extension for FluidExtension {
    fn new() -> Self {
        Self {
            cached_server_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        if language_server_id.as_ref() != Self::SERVER_ID {
            return Err(format!(
                "unexpected language server id '{}'",
                language_server_id.as_ref()
            ));
        }

        self.server_command(language_server_id, worktree)
    }
}

zed::register_extension!(FluidExtension);

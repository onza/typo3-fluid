# TYPO3 Fluid for Zed

Fluid templates (`.fluid.html`, `.fluid.xml`, …) in Zed.

<br>

## What it does

Tree-sitter grammar for highlighting, outline, brackets, and indents (HTML / XML / JSON / Text). Snippets for common ViewHelpers. A small Node helper LSP — downloaded from this repo’s GitHub release as `fluid-lsp.tar.gz` — for ViewHelper completion (tag + inline), hover, schema checks, and live `fluid:analyze` when a TYPO3 project exposes it (Core on 14+, [fluid-companion](https://github.com/s2b/fluid-companion) on 12/13; DDEV-aware).

Classic `Resources/Private/**/*.html` is not auto-detected (Zed extensions only bind suffixes). Use the globs in [examples/recommended-settings.json](examples/recommended-settings.json).

<br>

## If a community LSP appears

Grammar and snippets stay here. Point `fluid-lsp` at another stdio binary if the community ships one. Until then, the release-downloaded helper is the default.

<br>

## Development

```bash
npm install
npx tree-sitter generate
npx tree-sitter test
cargo check
./tools/pack-fluid-lsp.sh
```

**Extensions → Install Dev Extension** → this folder. Node 18+. Release flow: [PUBLISHING.md](PUBLISHING.md).

<br>

## Contributing

[github.com/onza/typo3-fluid](https://github.com/onza/typo3-fluid) — issues and PRs welcome.

<br>

## License

[MIT](LICENSE)

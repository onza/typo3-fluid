# Publishing

How to get this on the [Zed marketplace](https://zed.dev/extensions).

Repo: https://github.com/onza/typo3-fluid  
Extension ID: `fluid` (can't change after first publish)

## Marketplace rules (must keep)

From [Zed publishing prerequisites](https://zed.dev/docs/extensions/publishing/prerequisites.html):

- **Do not bundle a language server** with the extension. Zed downloads
  `fluid-lsp.tar.gz` from a **GitHub Release** at runtime (`src/lib.rs`).
  `builtin/` is **source for packing only** — never the runtime path.
- Project CLIs (`fluid:analyze`, `fluid:schema:generate`) come from the host
  TYPO3 project, not from this extension.
- Grammar `repository` must be the **HTTPS** GitHub URL + commit `rev`
  (never `file://` in a marketplace release).
- Snippets / Tree-sitter / language configs may ship in the extension.

## Checklist

1. `npm run build && npx tree-sitter test && cargo check`
2. Bump version everywhere it must match:
   `extension.toml`, `Cargo.toml`, `package.json`, `tree-sitter.json`
3. Commit and push (include `src/parser.c`, `src/scanner.c`, `src/tag.h`,
   `src/node-types.json`, `src/tree_sitter/`). Do **not** commit `dist/` or
   `fluid-lsp-*/` (gitignored — those are download/cache artifacts).
4. Set `[grammars.fluid].rev` in `extension.toml` to that commit SHA, push again.
5. **Pack and publish the Node helper as a GitHub Release asset** (required):

```bash
./tools/pack-fluid-lsp.sh
VERSION=$(grep '^version' extension.toml | head -1 | cut -d'"' -f2)   # e.g. 0.1.3
gh release create "v${VERSION}" \
  --title "v${VERSION}" \
  --notes "Fluid Zed extension ${VERSION}" \
  "dist/fluid-lsp.tar.gz#fluid-lsp.tar.gz"
```

   If the tag/release already exists, only upload the asset:

```bash
./tools/pack-fluid-lsp.sh
gh release upload "v${VERSION}" "dist/fluid-lsp.tar.gz#fluid-lsp.tar.gz" --clobber
```

6. PR to [zed-industries/extensions](https://github.com/zed-industries/extensions):
   - submodule `extensions/fluid` → `https://github.com/onza/typo3-fluid`
   - entry in `extensions.toml`:

```toml
[fluid]
submodule = "extensions/fluid"
version = "0.1.3"
```

   - `pnpm sort-extensions`

## Local Dev Extension

Marketplace installs always download the Release asset. For local testing either:

1. Create/upload the `v{version}` release (same as step 5), then reload the Dev Extension, or
2. Manually unpack into the extension work dir (gitignored):

```bash
./tools/pack-fluid-lsp.sh
VERSION=$(grep '^version' extension.toml | head -1 | cut -d'"' -f2)
mkdir -p "fluid-lsp-${VERSION}"
tar -xzf dist/fluid-lsp.tar.gz -C "fluid-lsp-${VERSION}"
```

Do **not** point the runtime at `builtin/server.js` — that path is intentionally unused.

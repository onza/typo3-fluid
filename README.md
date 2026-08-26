# TYPO3 Fluid for Zed

Fluid templates (`.fluid.html`, `.fluid.xml`, …) in Zed.

<br>

## What it does

Tree-sitter grammar for highlighting, outline, brackets, and indents (HTML / XML / JSON / Text). Snippets for common ViewHelpers. A small Node helper LSP — downloaded from this repo’s GitHub release as `fluid-lsp.tar.gz` — for ViewHelper completion (tag + inline), hover, schema checks, and live `fluid:analyze` when a TYPO3 project exposes it (Core on 14+, [fluid-companion](https://github.com/s2b/fluid-companion) on 12/13; DDEV-aware).

<br>

## Older templates without `.fluid.html`

Zed extensions can only bind fixed suffixes (e.g. `.fluid.html`). Classic TYPO3 paths like `Resources/Private/Templates/**/*.html` are **not** detected automatically.

Workaround — put this in your Zed settings (project or user), or copy from [examples/recommended-settings.json](examples/recommended-settings.json):

```json
{
  "file_types": {
    "Fluid HTML": [
      "**/Resources/Private/Templates/**/*.html",
      "**/Resources/Private/Layouts/**/*.html",
      "**/Resources/Private/Partials/**/*.html",
      "**/Resources/Private/Components/**/*.html",
      "**/Resources/Private/PageView/**/*.html",
      "**/ContentBlocks/**/templates/**/*.html",
      "*.fluid.html",
      "*.fluid.htm"
    ],
    "Fluid Text": [
      "**/Resources/Private/Templates/**/*.txt",
      "**/Resources/Private/Layouts/**/*.txt",
      "**/Resources/Private/Partials/**/*.txt",
      "**/Resources/Private/Components/**/*.txt",
      "**/Resources/Private/PageView/**/*.txt",
      "*.fluid.txt"
    ]
  }
}
```

Adjust the globs if your layout differs. Files matching these patterns get Fluid highlighting, snippets, and the helper LSP.

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

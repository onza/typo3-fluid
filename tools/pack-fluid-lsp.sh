#!/usr/bin/env bash
# Pack builtin/server.js + schemas into dist/fluid-lsp.tar.gz for GitHub Releases.
# Zed downloads this asset at runtime — it must not be embedded in the Wasm.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
src="$root/builtin"
out="$root/dist"
asset="$out/fluid-lsp.tar.gz"

if [[ ! -f "$src/server.js" ]]; then
  echo "missing $src/server.js" >&2
  exit 1
fi
if ! compgen -G "$src/schemas/schema_*.json" >/dev/null; then
  echo "missing $src/schemas/schema_*.json — run ./tools/generate-viewhelper-schemas.sh" >&2
  exit 1
fi

mkdir -p "$out"
rm -f "$asset"
# Flat archive: server.js + schemas/ at the root (extracted into fluid-lsp-{version}/)
tar -czf "$asset" -C "$src" server.js schemas
echo "Wrote $asset"
tar -tzf "$asset" | head -20

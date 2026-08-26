#!/usr/bin/env bash
# Regenerate bundled ViewHelper schemas (pipeline adapted from FriendsOfTYPO3/vscode-fluid-language).
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
gen="$root/tools/viewhelper-schemas"
out="$root/builtin/schemas"

cd "$gen"
composer install --no-interaction
composer generate
mkdir -p "$out"
cp -f "$gen/out"/schema_*.json "$out/"
echo "Wrote schemas to $out"
python3 -c "
import json
from pathlib import Path
out = Path('$out')
total = 0
for p in sorted(out.glob('schema_*.json')):
    n = len(json.loads(p.read_text())['tags'])
    total += n
    print(f'  {p.name}: {n} tags')
print(f'  total: {total}')
"

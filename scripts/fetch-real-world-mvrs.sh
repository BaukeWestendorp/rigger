#!/usr/bin/env bash
# Downloads the real-world MVR files listed in tests/data/mvr/real-world/manifest.tsv and checks
# their SHA-256 checksums. Files that are already present and valid are skipped.
set -euo pipefail

dir="$(cd "$(dirname "$0")/../tests/data/mvr/real-world" && pwd)"

if command -v sha256sum > /dev/null; then
    sha256() { sha256sum "$1" | cut -d' ' -f1; }
else
    sha256() { shasum -a 256 "$1" | cut -d' ' -f1; }
fi

failed=0
while IFS=$'\t' read -r file hash _fixtures _unresolved url; do
    [[ -z "$file" || "$file" == \#* ]] && continue
    path="$dir/$file"

    if [[ -f "$path" && "$(sha256 "$path")" == "$hash" ]]; then
        echo "ok      $file"
        continue
    fi

    if ! curl -fsSL --retry 3 -o "$path.part" "$url"; then
        echo "FAILED  $file: could not download $url" >&2
        rm -f "$path.part"
        failed=1
        continue
    fi

    actual="$(sha256 "$path.part")"
    if [[ "$actual" != "$hash" ]]; then
        echo "FAILED  $file: expected sha256 $hash, got $actual" >&2
        rm -f "$path.part"
        failed=1
        continue
    fi

    mv "$path.part" "$path"
    echo "fetched $file"
done < "$dir/manifest.tsv"

exit "$failed"

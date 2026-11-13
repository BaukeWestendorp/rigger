# Real-world MVR files

These files are exports from real tools. They are **not committed** to this repository: they are
large (about 70 MB together), most come without a license, and every one of them embeds GDTF files
from fixture manufacturers. Instead, `manifest.tsv` lists where to download each file and its
SHA-256 checksum.

To run the test:

```sh
scripts/fetch-real-world-mvrs.sh
cargo test --test load_real_world_mvr -- --ignored
```

CI runs the same steps in the `test-real-world-mvrs` job of `.github/workflows/rust.yml` and
caches the downloads until `manifest.tsv` changes.

The test checks that every file loads, that the fixture count matches the manifest, and that every
fixture's `GDTFSpec` resolves to a GDTF in the archive, except for the ones the manifest lists as
unresolved.

## Provenance

All files were downloaded and checksummed on 2026-11-13.

| File | MVR | Producer | Source |
| --- | --- | --- | --- |
| `Small_Bar.mvr` | 1.6 | Visibox | Test exports published by SpaceAge TV |
| `Medium_Club.mvr` | 1.6 | Visibox | Test exports published by SpaceAge TV |
| `Large_Theatre.mvr` | 1.6 | Visibox | Test exports published by SpaceAge TV |
| `Arena.mvr` | 1.6 | Visibox | Test exports published by SpaceAge TV |
| `Circle_Stage.mvr` | 1.5 | Johannes Lutra | [MVR-Stash](https://github.com/JMLutra/MVR-Stash), via [gdtf.eu][gdtf-eu] |
| `Messy_Patch.mvr` | 1.5 | Johannes Lutra | [MVR-Stash](https://github.com/JMLutra/MVR-Stash), via [gdtf.eu][gdtf-eu] |
| `Basic_Festival.mvr` | 1.5 | Johannes Lutra | [MVR-Stash](https://github.com/JMLutra/MVR-Stash), via [gdtf.eu][gdtf-eu] |
| `Midsize_w_GP.mvr` | 1.5 | Johannes Lutra | [MVR-Stash](https://github.com/JMLutra/MVR-Stash), via [gdtf.eu][gdtf-eu] |
| `Key_Arena_Remake_MVR.mvr` | 1.4 | Xzidental | Discord, via [gdtf.eu][gdtf-eu] |
| `Template_Stage1.mvr` | 1.4 | Xzidental | Discord, via [gdtf.eu][gdtf-eu] |
| `Demoshow_grandMA3.mvr` | 1.5 | MA Lighting | [gdtf.eu][gdtf-eu] |
| `Demostage_MVR.mvr` | 1.5 | MA Lighting | [gdtf.eu][gdtf-eu] |
| `Simple_Show.mvr` | 1.5 | MA Lighting | [gdtf.eu][gdtf-eu] |
| `Capture_Demo.mvr` | 1.4 | Capture | [gdtf.eu][gdtf-eu] |

The [gdtf.eu MVR example files][gdtf-eu] page says: "The collection of files below has mostly been
found randomly on the internet. Where known, attribution is given." Neither that page nor MVR-Stash
states a license.

[gdtf-eu]: https://www.gdtf.eu/blog/mvr-example-files/

## Quirks these files exercise

- Every file uses 4x3 `Matrix` nodes (`{u}{v}{w}{o}`).
- The four Visibox files use `universe.address` DMX addresses. The others use absolute addresses.
- `Demoshow_grandMA3.mvr`, `Demostage_MVR.mvr`, `Simple_Show.mvr` and `Messy_Patch.mvr` end with a
  NUL byte after `</GeneralSceneDescription>`.
- None of the non-Visibox files has a non-empty `provider` attribute.
- `Demoshow_grandMA3.mvr`, `Demostage_MVR.mvr`, `Simple_Show.mvr`, `Messy_Patch.mvr`,
  `Midsize_w_GP.mvr` and `Basic_Festival.mvr` write some `GDTFSpec`s without the `.gdtf` extension,
  and these files plus `Circle_Stage.mvr` have fixtures with an empty `GDTFSpec`.
- `Midsize_w_GP.mvr` and `Messy_Patch.mvr` embed GDTFs with negative DMX values (`DMXFrom="-255/4"`).
- `Basic_Festival.mvr` references `Cluster S2`, but the archive only contains `Roxx@Cluster S2.gdtf`.

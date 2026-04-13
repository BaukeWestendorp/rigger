# TODO (MVR)

## Archive support
- [ ] Implement `Bundle::from_archive_with_options` (read zip entries)
  - [ ] Parse `GeneralSceneDescription.xml` from archive
  - [ ] Build `ResourceMap` from archive entries
  - [ ] Implement `resource_bytes()` for archive bundles

## Resource access improvements
- [ ] Add streaming API: `Bundle::open_resource(&ResourceKey) -> Option<impl Read>`
- [ ] Implement `resource_bytes()` via `open_resource()`

## Path-based apps (archive extraction)
- [ ] Add archive extraction policy (folder-only consumers):
  - [ ] `ExtractPolicy::Never | Lazy(cache_dir) | Always(cache_dir)`
  - [ ] Implement `resolve_path()` for archives when extraction is enabled

## Writing / round-trip
- [ ] Add `Bundle::write_to_folder(...)` (`GeneralSceneDescription.xml` + resources)
- [ ] Add `Bundle::write_to_archive(...)` (zip output)

## Higher-level API
- [ ] Add `mvr::resolved` layer that resolves references and provides ergonomic accessors/iterators
- [ ] Add optional validation in higher layer (missing/unreferenced resources, schema checks)

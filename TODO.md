# TODO (MVR)

- Add streaming API: `Bundle::open_resource(&ResourceKey) -> Option<impl Read>`
- Implement `resource_bytes()` via `open_resource()`
- Archive: `ExtractPolicy::Never | Lazy(cache_dir) | Always(cache_dir)`
- Add optional validation in `mvr::resolved` layer (missing resources, schema checks)

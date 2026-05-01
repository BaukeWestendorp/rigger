# rigger

A Rust library for reading [MVR](https://www.gdtf.eu/mvr/prologue/introduction/) and [GDTF](https://www.gdtf.eu/gdtf/prologue/introduction/) files.

> ⚠️ **Warning** > This library is in early development. APIs, features, and behavior may change frequently and without notice.

## Overview

*MVR* (My Virtual Rig) and *GDTF* (General Device Type Format) are open standards used to describe lighting rigs and fixtures in entertainment production. While MVR files contain scene and rig data, GDTF files define the specific characteristics and geometry of individual fixtures.

Because these formats support thousands of devices across multiple manufacturers, their data structures are large and rely heavily on optional fields. This means it's non-trivial to read commonly used data like the channel count of a fixture's DMX mode. This often makes directly reading the structures verbose and difficult to manage.

`rigger` abstracts this complexity by providing lookup tables and high-level helper functions. The goal is to let you extract the data you actually need without navigating the deep, nested specifications of the underlying XML. Though, if you want to manually find anything defined in the description files, you can.

## Progress
### MVR
- [x] Bundle loading from folder
- [x] Bundle loading from `.mvr` archive.
- [x] Test deserialization of `bundle` description.
- [ ] Test deserialization of `bundle` resources.
- [x] Test deserialization API of higher level `Mvr` data type and it's children.
- [ ] Test negative paths in deserialization
- [ ] Documentation
### GDTF
- [ ] Bundle loading from folder
- [ ] Bundle loading from `.mvr` archive.
- [ ] Test deserialization of `bundle` description.
- [ ] Test deserialization of `bundle` resources.
- [ ] Test API of higher level `Gdtf` data type and it's children.
- [ ] Documentation


## Contributing
Contributions are welcome. If you find a file that this library fails to parse correctly or want to request a feature or suggest a change, feel free to open an issue!

## License

This project is dual-licensed under:

- MIT License
- Apache License, Version 2.0

You may choose either license to govern your use of this project.
See the LICENSE-MIT and LICENSE-APACHE files for details.

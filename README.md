# An alternative to `std` library for `no_std` environment

[![Latest version](https://img.shields.io/crates/v/eroc_microstd.svg)](https://crates.io/crates/eroc_microstd)
[![Documentation](https://docs.rs/eroc_microstd/badge.svg)](https://docs.rs/eroc_microstd)
![License](https://img.shields.io/crates/l/eroc_microstd.svg)

This project is a part of [Eroc](https://github.com/radifun/eroc) library collection.

`eroc_microstd` provides an alternative implementation for many popular features of `std` library and allows other [Eroc](https://github.com/radifun/eroc) projects to be used in `no_std` environment.

## Features

`eroc_microstd` contains `core` from Rust standard library so it can be used as the drop-in replacement in `no_std` environment.

There are also additional features implemented or imported from `core` and `std` libraries. The complete list of features, their functional parity and test coverage are provided as follows:

| Name                        | Reference                   | Functional                                                | Test                                                      | Notes |
|:----------------------------|:----------------------------|:----------------------------------------------------------|:----------------------------------------------------------|:------|
| `error`                     | `core::error`               | ![](https://img.shields.io/badge/-complete-blue)          | ![](https://img.shields.io/badge/-not%20started-red)      | |
| `io`                        | `std::io`                   | ![](https://img.shields.io/badge/-in%20progress-yellow)   | ![](https://img.shields.io/badge/-not%20started-red)      | |
| `vec`                       | `std::vec`                  | ![](https://img.shields.io/badge/-in%20progress-yellow)   | ![](https://img.shields.io/badge/-not%20started-red)      | |

## License

This project is provided under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) license.

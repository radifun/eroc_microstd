// =================================================================================================
// Copyright (c) 2023 Viet-Hoa Do <doviethoa@doviethoa.com>
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// =================================================================================================

//! This module imports the source code of [`std::io`] and makes some alteration.
//!
//! Major functional changes:
//!   - [`std::io::Error`] supports custom error with arbitrary error data.
//!     It requires dynamic memory allocation, which does not always exist in `no_std`.
//!     For now the library only allows &'static str as the error data of custom error.
//!     In the future it can use `feature = alloc` to allow dynamic memory allocation.

use std::path;

mod importer;
use importer::*;

/// Imports and alters [`std::io`] module.
pub fn import(src_path: &path::Path, dst_path: &path::Path) {
    import_error(&src_path.join("error.rs"), &dst_path.join("error/mod.rs"));
    import_error_repr_unpacked(
        &src_path.join("error/repr_unpacked.rs"),
        &dst_path.join("error/repr_unpacked.rs"),
    );
}

/// Imports and alters [`std::io::error`] module.
fn import_error(src_path: &path::Path, dst_path: &path::Path) {
    let f = read_file(src_path);

    // Removes attributes that are only allowed in internal/built-in libraries.
    let f = remove_stable_attr(f);

    // Keeps all unstable `ErrorKind`.
    let f = remove_line(f, r##"^\s*#\[unstable\(feature = "io_error_more""##);
    let f = remove_line(f, r##"^\s*#\[unstable\(feature = "io_error_uncategorized""##);

    // Removes unstable features.
    let f = remove_unstable_features(f);

    // Removes `repr_bitpacked` module as it uses many unstable features.
    // Always uses `repr_unpacked` instead.
    let f = remove_line(f, r".*cfg\(.*target_pointer_width.*");
    let f = remove_line(f, r"(?:mod|use) repr_bitpacked.*");

    // Removes macro as it is unstable feature.
    // It will be implemented using macro_rules!, and put to the top of the file.
    let f = remove_block(f, &regex::escape("pub(crate) macro const_io_error("));

    let f = insert_to_beginning(
        f,
        &[
            r"",
            r"/// Create and return an `io::Error` for a given `ErrorKind` and constant",
            r"/// message. This doesn't allocate.",
            r"macro_rules! const_io_error {",
            r"    ($kind:expr, $message:expr $(,)?) => {",
            r"        $crate::io::error::Error::from_static_message({",
            r"            const MESSAGE_DATA: $crate::io::error::SimpleMessage =",
            r"                $crate::io::error::SimpleMessage::new($kind, $message);",
            r"            &MESSAGE_DATA",
            r"        })",
            r"    };",
            r"}",
            r"",
        ],
    );

    // Change custom kind to contain static string slice instead of `Box`.
    let f = replace_text(f, &regex::escape("Box<dyn error::Error + Send + Sync>"), "&'static str");
    let f = replace_text(
        f,
        &regex::escape("Box::new(Custom { kind, error })"),
        "Custom { kind, error }",
    );
    let f = replace_text(f, r"\(c\) => Some\(&(?:mut )?\*c.error\)", "(_) => None");
    let f = replace_text(f, r"\(c\) => c\.error\.(?:cause|source)\(\)", "(_) => None");
    let f = replace_text(f, &regex::escape("c.error.description()"), "c.error");

    write_file(f, dst_path);
}

/// Imports and alters [`std::io`]`error/repr_unpacked` module.
fn import_error_repr_unpacked(src_path: &path::Path, dst_path: &path::Path) {
    let f = read_file(src_path);

    // Somehow `Repr::new` is unused.
    let f = remove_block(f, r".*fn new\(");

    // Custom kind is now known at compile time, hence we don't need to use `Box` anymore.
    let f = replace_text(f, "Box<Custom>", "Custom");

    // Removes unused `Box`.
    let f = remove_line(f, "^use alloc::boxed::Box;");

    write_file(f, dst_path);
}

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

use std::path;

mod importer;
use importer::*;

pub fn import_error(src_path: &path::Path, dst_path: &path::Path) {
    let f = read_file(src_path);

    // Keeps function `type_id` of `Error` trait even though it is marked as unstable.
    // For some reasons it is used by other stable function: (dyn Error + 'static)::is.
    let f = BlockRegex::new(
        f,
        Some(r##"^\s*(?:#\[unstable|feature|reason|issue|\)\]).*"##),
        r##"^(\s*fn type_id.*)"##,
        None,
        &["${1}"],
    );

    let f = remove_line(f, r##"^\s*#\[unstable\(feature = "error_type_id""##);

    // Removes attributes that are allowed in internal/built-in libraries.
    let f = remove_stable_attr(f);
    let f = remove_doc_attr(f);

    let f = remove_attr(f, "rustc_diagnostic_item");
    let f = remove_attr(f, "rustc_has_incoherent_inherent_impls");

    // Removes unstable features.
    let f = remove_unstable_features(f);
    let f = remove_attr(f, "unstable");

    // Other things
    let f = remove_block(f, r"impl Error for crate::char::ParseCharError"); // Use unstable feature.
    let f = remove_block(f, r"impl Error for crate::ffi::FromBytesWithNulError"); // Use unstable feature.
    let f = remove_fn(f, "provide"); // It is unstable feature of `Error` trait.

    let f = remove_text(f, "Demand, Provider, ");

    write_file(f, dst_path);
}

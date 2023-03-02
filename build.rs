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

use std::{env, path};

#[path = "import/error.rs"]
mod error;

fn main() {
    let out_path = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    let rustlib_path = path::PathBuf::from(env::var("RUSTLIB_PATH").unwrap());

    let core_path = rustlib_path.join("src/rust/library/core");
    let gen_path = out_path.join("rustlib");

    error::import_error(&core_path.join("src/error.rs"), &gen_path.join("src/error.rs"));
}

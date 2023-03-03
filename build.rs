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

#[path = "builder/core_error.rs"]
mod core_error;

#[path = "builder/std_io.rs"]
mod std_io;

fn main() {
    let out_path = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    let rustlib_path = path::PathBuf::from(env::var("RUSTLIB_PATH").unwrap());

    let std_path = rustlib_path.join("src/rust/library/std");
    let core_path = rustlib_path.join("src/rust/library/core");
    let gen_path = out_path.join("rustlib");

    core_error::import(&core_path.join("src/error.rs"), &gen_path.join("src/error.rs"));

    std_io::import(&std_path.join("src/io"), &gen_path.join("src/io"));
}

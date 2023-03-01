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

//! This module contains the merge of [`core`] and [`alloc`] libraries.

extern crate alloc as _1alloc;
extern crate core as _0core;

pub use _0core::*;

pub mod alloc {
    pub use super::_0core::*;
    pub use super::_1alloc::*;
}

pub mod borrow {
    pub use super::_0core::borrow::*;
    pub use super::_1alloc::borrow::*;
}

pub use _1alloc::boxed;
pub use _1alloc::collections;

pub mod ffi {
    pub use super::_0core::ffi::*;
    pub use super::_1alloc::ffi::*;
}

pub mod fmt {
    pub use super::_0core::fmt::*;
    pub use super::_1alloc::fmt::*;
}

pub use _1alloc::rc;

pub mod slice {
    pub use super::_0core::slice::*;
    pub use super::_1alloc::slice::*;
}

pub mod str {
    pub use super::_0core::str::*;
    pub use super::_1alloc::str::*;
}

pub use _1alloc::string::*;

pub mod sync {
    pub use super::_0core::sync::*;
    pub use super::_1alloc::sync::*;
}

pub mod task {
    pub use super::_0core::task::*;
    pub use super::_1alloc::task::*;
}

pub use _1alloc::format;
pub use _1alloc::vec;

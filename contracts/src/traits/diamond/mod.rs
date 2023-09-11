// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

#[allow(clippy::module_inception)]
mod diamond;

pub use diamond::*;
pub mod extensions {
    pub mod diamond_loupe;
}

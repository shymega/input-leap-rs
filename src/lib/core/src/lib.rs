// SPDX-FileCopyrightText: 2023 The Input-Leap Developers
//
// SPDX-License-Identifier: GPL-2.0-only

//! Core crate for `input-leap-rs`.
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::cargo,
    clippy::pedantic,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

pub mod reexports {
    //! Reexports of Input Leap-rs dependencies & componets.
    pub use input_leap_rs_platforms as platforms;
    pub use rmp;
    pub use rmp_serde;
}

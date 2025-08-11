//! This crate is used as a smoke test for the `typed_fields` crate. It uses some macros from the
//! crate to ensure that they compile in consuming crates without errors. To keep compile times
//! short, no optional features are enabled.

use typed_fields::{name, number};

name!(
    /// This tests the `name!` macro
    Test
);

number!(
    /// This tests the `number!` macro
    TestNumber
);

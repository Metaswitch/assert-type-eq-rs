//! Statically assert that types from potentially different crate versions via
//! different dependencies are identical.
//!
//! Until RFC 1977 (public dependencies) is accepted, the situation where
//! multiple different versions of the same crate are present is possible. In
//! most situations this will simply cause code to not compile, as types
//! mismatch, however with runtime structures like `TypeMap` this leads to
//! runtime errors (often silent!) instead. This macro allows compile-time
//! assertion that types via different dependencies are identical, and will
//! interoperate, which is easier to debug than runtime errors.
//!
//! Usage:
//!
//! ```
//! #[macro_use]
//! extern crate assert_type_eq;
//!
//! pub mod my_crate {
//!     pub struct MyStruct;
//! }
//!
//! mod a {
//!     pub use super::my_crate;
//! }
//!
//! mod b {
//!     pub use super::my_crate;
//! }
//!
//! assert_type_eq!(
//!     my_crate::MyStruct,
//!     a::my_crate::MyStruct,
//!     b::my_crate::MyStruct,
//! );
//!
//! fn main() {
//!     // ...
//! }
//! ```
//!
//! Specify all versions of the same type via different dependencies. Any types
//! that do not match the first type in the macro will cause a compile-time
//! error.

#[macro_export]
macro_rules! assert_type_eq {
    ( $t:ty, $( $ot:ty ),* $(,)* ) => {
        mod assert_type_eq_mod {
            #[allow(unused_imports)]
            use super::*;

            struct MatchingType<T>(T);

            #[allow(dead_code, unreachable_patterns)]
            fn assert_type_eq(mine: MatchingType<$t>) {
                match mine {
                    $( MatchingType::<$ot>(_) => () ),*
                }
            }
        }
    }
}

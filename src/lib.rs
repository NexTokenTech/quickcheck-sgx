//! This crate is a port of
//! [Haskell's QuickCheck](http://hackage.haskell.org/package/QuickCheck).
//!
//! For detailed examples, please see the
//! [README](https://github.com/BurntSushi/quickcheck).

#![cfg_attr(feature = "i128", feature(i128_type, i128))]

#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]
#![allow(deprecated)]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

use std::prelude::v1::*;

#[macro_use]
extern crate rand;
extern crate rand_core;

pub use arbitrary::{
    Arbitrary, Gen, StdGen, StdThreadGen,
    empty_shrinker, single_shrinker,
};
pub use rand_core::RngCore;
pub use tester::{QuickCheck, Testable, TestResult, quickcheck};

/// A macro for writing quickcheck tests.
///
/// This macro takes as input one or more property functions to test, and
/// produces a proper `#[test]` function for each property. If the property
/// fails, the behavior is as if `quickcheck` were called on the property
/// (i.e., it panics and fails the test).
///
/// Note that this macro doesn't support `mut` or patterns in parameters.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate quickcheck; fn main() {
/// quickcheck! {
///     fn prop_reverse_reverse(xs: Vec<usize>) -> bool {
///         let rev: Vec<_> = xs.clone().into_iter().rev().collect();
///         let revrev: Vec<_> = rev.into_iter().rev().collect();
///         xs == revrev
///     }
/// };
/// # }
/// ```
#[macro_export]
macro_rules! quickcheck {
    (@as_items $($i:item)*) => ($($i)*);
    {
        $(
            $(#[$m:meta])*
            fn $fn_name:ident($($arg_name:ident : $arg_ty:ty),*) -> $ret:ty {
                $($code:tt)*
            }
        )*
    } => (
        $crate::quickcheck! {
            @as_items
            $(
                //#[test]
                $(#[$m])*
                pub fn $fn_name() {
                    fn prop($($arg_name: $arg_ty),*) -> $ret {
                        $($code)*
                    }
                    $crate::quickcheck(prop as fn($($arg_ty),*) -> $ret);
                }
            )*
        }
    )
}

mod arbitrary;
mod tester;

#[cfg(test)]
mod tests;

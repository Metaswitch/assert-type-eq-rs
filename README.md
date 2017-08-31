assert-type-eq
==============

[![Build Status](https://travis-ci.org/Metaswitch/assert-type-eq-rs.svg?branch=master)](https://travis-ci.org/Metaswitch/assert-type-eq-rs)
[![crates.io](https://img.shields.io/crates/v/assert-type-eq.svg)](https://crates.io/crates/assert-type-eq)

This Rust crate adds a macro to assert at compile time that certain types are
the same. This is useful when using the same crate via different dependencies,
which may bring in different versions of the same crate, which Rust considers
to have incompatible types. In most circumstances this will by itself lead to
a compile-time error, however when using runtime structures such as `Any` or
[`TypeMap`](https://crates.io/crates/typemap), this will not cause compile-time
issues, but rather runtime problems with missing or unexpected data. By using
this crate, the different versions of the same crate can be asserted to be
compatible, turning a runtime error into a compile-time error.

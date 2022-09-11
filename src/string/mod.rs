#![deny(warnings)]
/// Provides demodulize a string.
///
/// Example string `Foo::Bar` becomes `Bar`
#[cfg(feature = "heavyweight")]
pub mod demodulize;
/// Provides deconstantizea string.
///
/// Example string `Foo::Bar` becomes `Foo`
#[cfg(feature = "heavyweight")]
pub mod deconstantize;
/// Provides conversion to plural strings.
///
/// Example string `FooBar` -> `FooBars`
#[cfg(feature = "heavyweight")]
pub mod pluralize;
/// Provides conversion to singular strings.
///
/// Example string `FooBars` -> `FooBar`
#[cfg(feature = "heavyweight")]
pub mod singularize;
/// Provides conversion to articlized strings.
///
/// Example string `user` -> `a user`
#[cfg(feature = "heavyweight")]
pub mod articlize;
/// Provides conversion to dearticlized strings.
///
/// Example string `a cat` -> `cat`
#[cfg(feature = "heavyweight")]
pub mod dearticlize;
mod constants;

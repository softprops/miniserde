//! *Prototype of a data structure serialization library with several opposite
//! design goals from [Serde](https://serde.rs).*
//!
//! As a prototype, this library is not a production quality engineering
//! artifact the way Serde is. At the same time, it is more than a proof of
//! concept and should be totally usable for the range of use cases that it
//! targets, which is qualified below.
//!
//! # Example
//!
//! ```rust
//! #[macro_use]
//! extern crate miniserde;
//!
//! use miniserde::json;
//!
//! #[derive(MiniSerialize, MiniDeserialize, Debug)]
//! struct Example {
//!     code: u32,
//!     message: String,
//! }
//!
//! fn main() -> miniserde::Result<()> {
//!     let example = Example {
//!         code: 200,
//!         message: "reminiscent of Serde".to_owned(),
//!     };
//!
//!     let j = json::to_string(&example);
//!     println!("{}", j);
//!
//!     let out: Example = json::from_str(&j)?;
//!     println!("{:?}", out);
//!
//!     Ok(())
//! }
//! ```
//!
//! #
//!
//! Here are some similarities and differences compared to Serde.
//!
//! ## <font color="#C0C0C0">Similar:</font> Stupidly good performance
//!
//! Seriously this library is way faster than it deserves to be. With very
//! little profiling and optimization so far and opportunities for improvement,
//! this library is on par with serde\_json for some use cases, slower by a
//! factor of 1.5 for most, and slower by a factor of 2 for some. That is
//! remarkable considering the other advantages below.
//!
//! ## <font color="#C0C0C0">Similar:</font> Strongly typed data
//!
//! Just like Serde, we provide a derive macro for a Serialize and Deserialize
//! trait. You derive these traits on your own data structures and use
//! `json::to_string` to convert any Serialize type to JSON and `json::from_str`
//! to parse JSON into any Deserialize type. Like serde\_json there is a `Value`
//! enum for embedding untyped components.
//!
//! ## <font color="#C0C0C0">Different:</font> Minimal design
//!
//! This library does not tackle as expansive of a range of use cases as Serde
//! does. Feature requests are practically guaranteed to be rejected. If your
//! use case is not already covered, please use Serde.
//!
//! The implementation is less code by a factor of 12 compared to serde +
//! serde\_derive + serde\_json, and less code even than the `json` crate which
//! provides no derive macro and cannot manipulate strongly typed data.
//!
//! ## <font color="#C0C0C0">Different:</font> No monomorphization
//!
//! There are no nontrivial generic methods. All serialization and
//! deserialization happens in terms of trait objects. Thus no code is compiled
//! more than once across different generic parameters. In contrast, serde\_json
//! needs to stamp out a fair amount of generic code for each choice of data
//! structure being serialized or deserialized.
//!
//! Without monomorphization, the derived impls compile lightning fast and
//! occupy very little size in the executable.
//!
//! ## <font color="#C0C0C0">Different:</font> No recursion
//!
//! Serde depends on recursion for serialization as well as deserialization.
//! Every level of nesting in your data means more stack usage until eventually
//! you overflow the stack. Some formats set a cap on nesting depth to prevent
//! stack overflows and just refuse to deserialize deeply nested data.
//!
//! In miniserde neither serialization nor deserialization involves recursion.
//! You can safely process arbitrarily nested data without being exposed to
//! stack overflows. Not even the Drop impl of our json `Value` type is
//! recursive so you can safely nest them arbitrarily.
//!
//! ## <font color="#C0C0C0">Different:</font> No deserialization error messages
//!
//! When deserialization fails, the error type is a unit struct containing no
//! information. This is a legit strategy and not just laziness. If your use
//! case does not require error messages, good, you save on compiling and having
//! your instruction cache polluted by error handling code. If you do need error
//! messages, then upon error you can pass the same input to serde\_json to
//! receive a line, column, and helpful description of the failure. This keeps
//! error handling logic out of caches along the performance-critical codepath.
//!
//! ## <font color="#C0C0C0">Different:</font> Infallible serialization
//!
//! Serialization always succeeds. This means we cannot serialize some data
//! types that Serde can serialize, such as `Mutex` which may fail to serialize
//! due to poisoning. Also we only serialize to `String`, not to something like
//! an i/o stream which may be fallible.
//!
//! ## <font color="#C0C0C0">Different:</font> JSON only
//!
//! The same approach in this library could be made to work for other data
//! formats, but it is not a goal to enable that through what this library
//! exposes.
//!
//! ## <font color="#C0C0C0">Different:</font> Structs only
//!
//! The miniserde derive macros will refuse anything other than a braced struct
//! with named fields. Enums and tuple structs are not supported.
//!
//! ## <font color="#C0C0C0">Different:</font> No customization
//!
//! Serde has tons of knobs for configuring the derived serialization and
//! deserialization logic through attributes. Or for the ultimate level of
//! configurability you can handwrite arbitrarily complicated implementations of
//! its traits.
//!
//! Miniserde provides just one attribute which is `rename`, and severely
//! restricts the kinds of on-the-fly manipulation that are possible in custom
//! impls. If you need any of this, use Serde -- it's a great library.

#![doc(html_root_url = "https://docs.rs/miniserde/0.1.7")]
#![cfg_attr(feature = "cargo-clippy", allow(renamed_and_removed_lints))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(cast_lossless, enum_variant_names, transmute_ptr_to_ptr)
)]

#[allow(unused_imports)]
#[macro_use]
extern crate mini_internal;
#[doc(hidden)]
pub use mini_internal::*;

extern crate itoa;
extern crate ryu;

// Not public API.
#[doc(hidden)]
pub mod export;

#[macro_use]
mod careful;

#[macro_use]
mod place;

mod error;
mod ignore;

pub mod de;
pub mod json;
pub mod ser;

#[doc(inline)]
pub use de::Deserialize;
pub use error::{Error, Result};
#[doc(inline)]
pub use ser::Serialize;

make_place!(Place);

#[allow(non_camel_case_types)]
struct private;

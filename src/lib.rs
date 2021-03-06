//! Memory-efficient data structures based on patricia tree (a.k.a, radix tree).
//!
//! A common prefixes of the keys in a patricia tree are represented by a shared path.
//! So if the prefixes of the key set is highly redundant,
//! the memory usage of the resulting patricia tree will be drastically less than
//! more generic data structures (e.g., `BTreeMap`).
//!
//! See [Radix tree](https://en.wikipedia.org/wiki/Radix_tree) for more details.
//!
//! # Examples
//!
//! ```
//! use patricia_tree::PatriciaMap;
//!
//! let mut map = PatriciaMap::new();
//! map.insert("foo", 1);
//! map.insert("bar", 2);
//! map.insert("baz", 3);
//! assert_eq!(map.len(), 3);
//!
//! assert_eq!(map.get("foo"), Some(&1));
//! assert_eq!(map.get("bar"), Some(&2));
//! assert_eq!(map.get("baz"), Some(&3));
//! ```
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]

#[macro_use]
extern crate bitflags;
#[cfg(feature = "binary-format")]
#[macro_use]
extern crate bytecodec;
extern crate libc;
#[cfg(test)]
extern crate rand;
#[cfg(feature = "binary-format")]
#[macro_use]
extern crate trackable;

pub use map::PatriciaMap;
pub use set::PatriciaSet;

pub mod map;
pub mod node;
pub mod set;

#[cfg(feature = "binary-format")]
mod codec;
mod tree;

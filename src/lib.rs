#![deny(unused_extern_crates, unused_must_use)]
#![forbid(intra_doc_link_resolution_failure)]

//! A simple crate for reading the data files of FFXIV.
//!
//! The main entry point for most use cases is the `SqPack` struct,
//! which accepts a path to the data files on the user's machine.
//! From an instance of this struct, you can open `Read` implementors
//! which return the decoded data.
//!
//! For more advanced usage, the internal mechanisms are exposed. Typically
//! one would start with a `SqPath` / `SqPathBuf`, pass that to a
//! function in the `io` module, along with a path to the data files, to
//! open `Read` streams.

extern crate byteorder;

/// Types and functions related to locating data within the Sqpack
pub mod sqpath;

/// Types and functions related to reading data from the Sqpack
pub mod io;

/// Functions relating to hashing data in the same way as FFXIV to assist locating files.
pub mod hash;
mod hash_consts;

pub use io::{
    SqpackError,
    SqResult,
    sqfile::SqFile
};
pub use sqpath::{
    SqPath
};
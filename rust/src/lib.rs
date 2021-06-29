#![deny(clippy::all)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::upper_case_acronyms)]

pub mod bls;
pub mod proofs;
pub mod util;

// global allocator
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

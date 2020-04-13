#![recursion_limit = "1024"]
#![cfg_attr(feature = "bench", feature(test))]

#[macro_use]
extern crate chalk_macros;

pub mod db;
pub mod error;
pub mod lowering;
pub mod program;
pub mod program_environment;
pub mod query;

use chalk_ir::interner::ChalkIr;
pub use chalk_ir::interner::{Identifier, RawId};
use chalk_ir::Binders;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeSort {
    Struct,
    Trait,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeKind {
    pub sort: TypeSort,
    pub name: Identifier,
    pub binders: Binders<ChalkIr, ()>,
}

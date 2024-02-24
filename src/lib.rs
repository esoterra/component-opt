mod module;
mod component;

use thiserror::Error;
use wasmparser::BinaryReaderError;
use wasm_opt::OptimizationError;

#[derive(Error, Debug)]
pub enum ComponentOptError {
    #[error("Error in wasm-opt")]
    WasmOpt(#[from] OptimizationError),
    #[error("IO error")]
    IO(#[from] std::io::Error),
    #[error("Unknown section type {section:?}")]
    UnknownSection { section: String },
    #[error("Error reading component binary")]
    BinaryReader(#[from] BinaryReaderError)
}

pub use component::{optimize_bytes, optimize_file};

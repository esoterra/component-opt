use wasm_opt::{Feature, OptimizationOptions, ShrinkLevel};
use std::{env, fs};
use std::path::Path;
use uuid::Uuid;

use crate::ComponentOptError;

pub(crate) fn optimize_bytes(input_bytes: &[u8]) -> Result<Vec<u8>, ComponentOptError> {
    // Choose temporary file names using UUID
    let id = Uuid::new_v4();
    let dir = env::temp_dir();
    let tmp_input = dir.join(format!("claw.opt.input.{}.wasm", id));
    let tmp_output = dir.join(format!("claw.opt.output.{}.wasm", id));
    // Write to temp input
    fs::write(&tmp_input, &input_bytes)?;
    // Optimize
    optimize_file(&tmp_input, &tmp_output)?;
    // Read bytes from temp output
    let bytes = fs::read(&tmp_output)?;
    // Cleanup temporary files
    fs::remove_file(&tmp_input)?;
    fs::remove_file(&tmp_output)?;
    // Return output bytes
    Ok(bytes)
}

fn optimize_file(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Result<(), ComponentOptError> {
    OptimizationOptions::new_opt_level_2()
        .shrink_level(ShrinkLevel::Level1)
        .enable_feature(Feature::All)
        .run(&input_path, &output_path)?;
    Ok(())
}
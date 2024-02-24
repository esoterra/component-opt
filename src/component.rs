use std::{fs, path::Path};

use wasmparser::{Parser, Payload::*};
use wasm_encoder::{Component, RawSection, ComponentSectionId};

use crate::{module, ComponentOptError};

pub fn optimize_bytes(input_bytes: Vec<u8>) -> Result<Vec<u8>, ComponentOptError> {
    let parser = Parser::new(0);
    let mut output = Component::new();

    for payload in parser.parse_all(input_bytes.as_slice()) {
        let payload = payload?;
        match payload {
            ModuleSection { parser, range } => {
                let id = ComponentSectionId::CoreModule.into();
                let module_bytes = &input_bytes.as_slice()[range];
                for _ in parser.parse_all(module_bytes) {
                    // DO NOTHING
                }
                let module_bytes = module::optimize_bytes(module_bytes)?;
                output.section(&RawSection { id, data: module_bytes.as_slice() });
            },

            // Skip all Module sections
            Version { .. } => { /* ... */ }
            TypeSection(_) => { /* ... */ }
            ImportSection(_) => { /* ... */ }
            FunctionSection(_) => { /* ... */ }
            TableSection(_) => { /* ... */ }
            MemorySection(_) => { /* ... */ }
            TagSection(_) => { /* ... */ }
            GlobalSection(_) => { /* ... */ }
            ExportSection(_) => { /* ... */ }
            StartSection { .. } => { /* ... */ }
            ElementSection(_) => { /* ... */ }
            DataCountSection { .. } => { /* ... */ }
            DataSection(_) => { /* ... */ }
            CodeSectionStart { .. } => { /* ... */ }
            CodeSectionEntry(_) => { /* ... */ }

            payload => {
                match payload.as_section() {
                    Some((id, range)) => {
                        let data = &input_bytes.as_slice()[range];
                        output.section(&RawSection { id, data });
                    },
                    None => {
                        // No op, should only ever be version which we already encode
                    }
                }
            }
        }
    }
    Ok(output.finish())
}

pub fn optimize_file(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Result<(), ComponentOptError> {
    let input_bytes = fs::read(input_path)?;
    let output_bytes = optimize_bytes(input_bytes)?;
    fs::write(output_path, output_bytes)?;
    Ok(())
}
use ndx_core::{parse_document, MAX_DOCUMENT_BYTES};
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process::ExitCode;

fn run() -> Result<(), (u8, String)> {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_default();
    let Some(path) = args.next() else {
        return Err((2, format!("usage: {} <document.ndx.json>", program.to_string_lossy())));
    };
    if args.next().is_some() {
        return Err((2, format!("usage: {} <document.ndx.json>", program.to_string_lossy())));
    }

    let file = File::open(&path).map_err(|e| (2, format!("cannot open {}: {e}", path.to_string_lossy())))?;
    let mut bytes = Vec::new();
    file.take((MAX_DOCUMENT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| (2, format!("cannot read {}: {e}", path.to_string_lossy())))?;

    match parse_document(&bytes) {
        Ok(document) => {
            println!("VALID (experimental NDX model {})", document.model_version);
            Ok(())
        }
        Err(error) => Err((1, error.to_string())),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err((code, message)) => {
            eprintln!("{message}");
            ExitCode::from(code)
        }
    }
}

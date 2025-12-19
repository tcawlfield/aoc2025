use anyhow::{Context as _, Result};
use std::path::{Path, PathBuf};

pub fn get_input(filename: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("input")
        .join(filename)
}

pub fn get_input_string(filename: &str) -> Result<String> {
    let file_path = get_input(filename);
    std::fs::read_to_string(&file_path).with_context(|| format!("Reading file {}", filename))
}

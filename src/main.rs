use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let bytes = std::fs::read("4Written Assign 4.pdf").unwrap();
    let out: String = pdf_extract::extract_text_from_mem(&bytes).unwrap();

    let mut file = File::create("pdf_text.txt");

    file?.write_all(out.as_bytes())?;
    Ok(())
}

fn remove_whitespace(input: String) -> String {
    return input.chars().filter(|&c| !c.is_whitespace()).collect();
}
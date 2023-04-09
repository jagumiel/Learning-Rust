use std::fs::File;
use std::io::{BufRead, BufReader, Result, Write};

fn main() -> Result<()> {
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    let mut output_file = File::create("output.txt")?;

    for line in reader.lines() {
        let upper_line = line?.to_uppercase();
        writeln!(output_file, "{}", upper_line)?;
    }

    Ok(())
}
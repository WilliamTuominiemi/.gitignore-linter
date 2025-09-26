use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open(".gitignore")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.split("\n").collect();
    println!("{:?}", lines);
    
    let mut row = 0;
    while row < lines.len() {
        let line = lines[row];
        if line != line.trim() {
            println!("Trailing whitespace on row {}", row);
            println!("--> {}", line);
        }
        row += 1;
    }

    Ok(())
}


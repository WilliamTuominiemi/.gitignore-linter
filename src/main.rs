use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open(".gitignore")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.split("\n").collect();
        
    let mut unique_lines = Vec::new();

    let mut row = 0;
    while row < lines.len() {
        let line = lines[row];
        if line != line.trim() {
            println!("Trailing whitespace on row {}", row);
            println!("--> |{}|", line);
        }

        if unique_lines.contains(&line) {
            if line != "" {
                println!("Duplicate rule on row {}", row);
                println!("--> |{}|", line);
            }
        } else {
            unique_lines.push(line);
        }

        if line.contains('[') && !line.contains(']') {
            println!("Match square bracket not closed on row {}", row);
            println!("--> |{}|", line);
        } else if line.contains(']') && !line.contains('[') {
            println!("Match square bracket not opened on row {}", row);
            println!("--> |{}|", line);
        }

        if line.contains('\\') {
            let pos = line.chars().position(|c| c == '\\').unwrap();
            let escaped_char = line.chars().nth(pos + 1);
            
            match escaped_char {
                Some(c) => {
                    if !['#', '!', '[', ']', '*', '?', '\\'].contains(&c) {
                        println!("\\ used for escaping non special character on row {}", row);
                        println!("--> |{}|", line);
                    }
                }
                None => {
                    println!("Escaping emptyness on row {}", row);
                    println!("--> |{}|", line);
                }
            }
        }

        row += 1;
    }

    Ok(())
}


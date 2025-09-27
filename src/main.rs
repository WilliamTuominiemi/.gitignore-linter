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
            log_issue("Trailing whitespace", row, line);
        }

        if unique_lines.contains(&line) {
            if line != "" {
                log_issue("Duplicate rule", row, line);
            }
        } else {
            unique_lines.push(line);
        }

        if line.contains('[') && !line.contains(']') {
            log_issue("Match square bracket not closed", row, line);
        } else if line.contains(']') && !line.contains('[') {
            log_issue("Match square bracket not opened", row, line);
        }

        if line.contains('\\') {
            let pos = line.chars().position(|c| c == '\\').unwrap();
            let escaped_char = line.chars().nth(pos + 1);
            
            match escaped_char {
                Some(c) => {
                    if !['#', '!', '[', ']', '*', '?', '\\'].contains(&c) {
                        log_issue("\\ used for escaping non special character", row, line);
                    }
                }
                None => {
                    log_issue("Escaping emptyness", row, line);
                }
            }
        }

        row += 1;
    }

    Ok(())
}

fn log_issue(msg: &str, row: usize, line: &str) {
    println!("{} on row {}", msg, row);
    println!("--> |{}|", line);
}

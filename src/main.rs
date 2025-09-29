use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open(".gitignore")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.lines().collect();

    linter(lines);

    Ok(())
}

fn linter(lines: Vec<&str>) {
    let mut unique_lines = Vec::new();

    let mut row = 0;
    while row < lines.len() {
        let line = lines[row];

        match lint(line) {
            Some(msg) => log_issue(msg, row, line),
            None => (),
        }

        if unique_lines.contains(&line) {
            if line != "" {
                log_issue("Duplicate rule", row, line);
            }
        } else {
            unique_lines.push(line);
        }

        row += 1;
    }
}

fn lint(line: &str) -> Option<&str> {
    if line != line.trim() {
        return Some("Trailing whitespace");
    }

    if line.contains('[') && !line.contains(']') {
        return Some("Match square bracket not closed");
    } else if line.contains(']') && !line.contains('[') {
        return Some("Match square bracket not opened");
    }

    if line.contains('\\') {
        let pos = line.chars().position(|c| c == '\\').unwrap();
        let escaped_char = line.chars().nth(pos + 1);

        match escaped_char {
            Some(c) => {
                if !['#', '!', '[', ']', '*', '?', '\\'].contains(&c) {
                    return Some("\\ used for escaping non special character");
                }
            }
            None => {
                return Some("Escaping emptiness");
            }
        }
    }

    return None;
}

fn log_issue(msg: &str, row: usize, line: &str) {
    println!("{} on row {}", msg, row);
    println!("--> {}", line);
}

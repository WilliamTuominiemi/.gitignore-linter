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

#[test]
fn trailing_whitespace_rule() {
    let line1 = "   test.txt   ";
    let line2 = "test.txt   ";
    let line3 = "   test.txt";
    let line4 = "test.txt";
    let line5 = "te  st.txt";

    let lint_msg = "Trailing whitespace";

    assert_eq!(lint(line1), Some(lint_msg));
    assert_eq!(lint(line2), Some(lint_msg));
    assert_eq!(lint(line3), Some(lint_msg));
    assert_eq!(lint(line4), None);
    assert_eq!(lint(line5), None);
}

#[test]
fn match_square_bracket_not_closed() {
    let line1 = "test.txt";
    let line2 = "test[0-9].txt";
    let line3 = "test[0-9.txt";

    let lint_msg = "Match square bracket not closed";

    assert_eq!(lint(line1), None);
    assert_eq!(lint(line2), None);
    assert_eq!(lint(line3), Some(lint_msg));
}

#[test]
fn match_square_bracket_not_opened() {
    let line1 = "test.txt";
    let line2 = "test[0-9].txt";
    let line3 = "test0-9].txt";

    let lint_msg = "Match square bracket not opened";

    assert_eq!(lint(line1), None);
    assert_eq!(lint(line2), None);
    assert_eq!(lint(line3), Some(lint_msg));
}

#[test]
fn escape_non_special_characters() {
    let line1 = "\\#";
    let line2 = "\\a";
    let line3 = "\\0";

    let lint_msg = "\\ used for escaping non special character";

    assert_eq!(lint(line1), None);
    assert_eq!(lint(line2), Some(lint_msg));
    assert_eq!(lint(line3), Some(lint_msg));
}

#[test]
fn escaping_emptiness() {
    let line1 = "test\\";
    let line2 = "\\";

    let lint_msg = "Escaping emptiness";

    assert_eq!(lint(line1), Some(lint_msg));
    assert_eq!(lint(line2), Some(lint_msg));
}

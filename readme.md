# Gitigli - .gitignore linter

A linter for your .gitignore file.

## Installation and usage
*Rust and Cargo are required to be installed*

Run ``cargo install --path .`` in this project.

Then in your project where you wan't to lint your .gitignore run ``gitigli``.

## Example output

```
\ used for escaping non special character on row 12
--> |docs\file.md|
```

## Checks

- Trailing whitespace
- Duplicate rule
- Match square bracket issues
- Wrong \ usage
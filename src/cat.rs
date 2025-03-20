// TODO [koopa] basics of cat are print the input file to stdout
// rundown of gnu version: https://www.gnu.org/software/coreutils/manual/html_node/cat-invocation.html#cat-invocation

use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Not;

// main option types:
// -- `-b` number non-empty output lines
// -- `-E` add $ to the end of each line
// -- `-n` number all output lines, ignored if `-b` present
// -- `-s` ignore adjacent blank lines
// -- `-T` display tabs as `^I`
// -- `-v` display non-printing characters (except tab and lfd)
// not gonna do -v because I don't really know how it works

enum LineNumberStyle {
    None,
    All,
    NonEmpty,
}

use clap::Parser;
use patharg::InputArg;

#[derive(Parser, Debug)]
pub struct CatOptions {
    /// Number non-blank lines starting at 1.
    #[arg(short = 'b')]
    number_non_blank: bool,
    /// Display a `$` after the end of each line. The \r\n combination is shown as `^M$`.
    #[arg(short = 'E')]
    show_line_end: bool,
    /// Number all output lines, starting with 1. This option is ignored if -b is in effect.
    #[arg(short = 'n')]
    number_all_lines: bool,
    /// Suppress repeated adjacent blank lines; output just one empty line instead of several.
    #[arg(short = 's')]
    ignore_adjacent_blanks: bool,
    /// Display TAB characters as `^I`.
    #[arg(short = 'T')]
    display_tab_symbol: bool,
    /// Ignored; for POSIX compatability.
    #[arg(short = 'u')]
    do_nothing: bool,
    /// File to operate on, defaults to STDIN.
    #[arg(value_name = "FILE", default_value_t)]
    path: InputArg,
}

pub fn kitty(cat_options: CatOptions) -> io::Result<()> {
    let mut f = BufReader::new(cat_options.path.open()?);

    let mut line_number_style = LineNumberStyle::None;
    if cat_options.number_non_blank {
        line_number_style = LineNumberStyle::NonEmpty;
    } else if cat_options.number_all_lines {
        line_number_style = LineNumberStyle::All;
    }

    let mut line_number: usize = 0;
    let mut previous_line_blank = false;
    let mut line = String::new();

    loop {
        line.clear();
        if f.read_line(&mut line)? == 0 {
            break;
        }

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            if previous_line_blank && cat_options.ignore_adjacent_blanks {
                continue;
            }
            previous_line_blank = true;
        } else {
            previous_line_blank = false;
        }

        let prefix = match line_number_style {
            LineNumberStyle::None => "".to_string(),
            LineNumberStyle::All => {
                line_number += 1;
                format!("\t{} ", line_number)
            }
            LineNumberStyle::NonEmpty => {
                if line.trim().is_empty().not() {
                    line_number += 1;
                    format!("\t{} ", line_number)
                } else {
                    "".to_string()
                }
            }
        };

        let new_line_pos = match line.rfind('\n') {
            Some(nlp) => nlp,
            None => line.len(),
        };

        if cat_options.show_line_end {
            line.insert(new_line_pos, '$');
        }

        line = match cat_options.display_tab_symbol {
            true => line.replace('\t', "^I"),
            false => line,
        };

        print!("{}{}", prefix, line);
    }

    Ok(())
}

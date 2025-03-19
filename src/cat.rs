// TODO [koopa] basics of cat are print the input file to stdout
// rundown of gnu version: https://www.gnu.org/software/coreutils/manual/html_node/cat-invocation.html#cat-invocation

use std::io;
use std::io::BufReader;
use std::io::prelude::*;

// main option types:
// -- `-b` number non-empty output lines
// -- `-E` add $ to the end of each line
// -- `-n` number all output lines, ignored if `-b` present
// -- `-s` ignore adjacent blank lines
// -- `-T` display tabs as `^I`
// -- `-v` display non-printing characters (except tab and lfd)

// enum LineNumberStyle {
//     None,
//     All,
//     NonEmpty,
// }

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
    /// Display control characters except for LFD and TAB using `^` notation and precede characters that have the high bit set with `M-`.
    #[arg(short = 'v')]
    display_unprintables: bool,
    /// File to operate on, defaults to STDIN.
    #[arg(value_name = "FILE", default_value_t)]
    path: InputArg,
}

pub fn kitty(cat_options: CatOptions) -> io::Result<()> {
    let f = BufReader::new(cat_options.path.open()?);

    let line_suffix = match cat_options.show_line_end {
        true => "$",
        false => "",
    };

    for (line_number, line) in f.lines().enumerate() {
        let prefix = match cat_options.number_all_lines {
            true => format!("{} ", line_number + 1),
            false => "".to_string(),
        };
        print!("{}{}{}\n", prefix, line?, line_suffix);
    }

    Ok(())
}

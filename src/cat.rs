// TODO [koopa] basics of cat are print the input file to stdout
// rundown of gnu version: https://www.gnu.org/software/coreutils/manual/html_node/cat-invocation.html#cat-invocation

use std::fs::File;
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

pub fn kitty() -> io::Result<()> {
    let file_name = "emoji_test.txt";
    let f = BufReader::new(File::open(file_name)?);

    let show_ends = false;
    let line_ending = match show_ends {
        true => "$",
        false => "",
    };

    for line in f.lines() {
        print!("{}{}\n", line?, line_ending);
    }

    Ok(())
}

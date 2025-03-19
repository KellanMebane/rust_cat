use std::io;

mod cat;
use cat::CatOptions;

use clap::Parser;

fn main() -> io::Result<()> {
    let cat_options = CatOptions::parse();

    cat::kitty(cat_options)?;

    Ok(())
}

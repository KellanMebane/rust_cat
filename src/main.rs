use std::io;

mod cat;

fn main() -> io::Result<()> {
    cat::kitty()?;
    Ok(())
}

use std::env::args;

use parser::parse;
use ui::compile;

fn help() {
    println!(
        "
Usage
-----

parrot <file path>

example: parrot code.echo

For more information see https://github.com/togglebyte/parrot
"
    );
}

fn main() -> anyhow::Result<()> {
    let mut args = args().skip(1);
    let Some(path) = args.next() else {
        help();
        return Ok(());
    };

    let code = std::fs::read_to_string(path)?;
    let instructions = parse(&code)?;
    let instructions = compile(instructions)?;
    ui::run(instructions);
    Ok(())
}

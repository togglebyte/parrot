use std::env::args;

use parser::parse;
use ui::compile;

fn help() {
    println!(
        "
Usage
-----

run:            parrot <file path>
print syntaxes: parrot --syntax
print themes:   parrot --themes

example: parrot code.echo

For more information see https://github.com/togglebyte/parrot
"
    );
}

fn main() -> anyhow::Result<()> {
    let mut args = args().skip(1);
    let Some(arg) = args.next() else {
        help();
        return Ok(());
    };

    ui::setup_paths::ensure_exists()?;

    if arg == "--syntax" {
        ui::print_syntaxes();
        return Ok(());
    }

    if arg == "--themes" {
        ui::print_themes();
        return Ok(());
    }

    let echo = std::fs::read_to_string(arg)?;
    let instructions = parse(&echo)?;
    let instructions = compile(instructions)?;
    ui::run(instructions)?;
    Ok(())
}

use std::env::args;
use parser::parse;

fn help() {
    println!("
Usage
-----

parrot <file path>
or
parrot <file path> <comment prefix>

example: play back code.echo with `#` as the comments
parrot code.echo #

For more information see https://github.com/togglebyte/parrot
");
}

fn main() -> anyhow::Result<()> {
    let mut args = args().skip(1);
    let Some(path) = args.next() else {
        help();
        return Ok(());
    };

    // let path = "/media/rustvids/anathema/hackbar/i3.echo";

    let code = std::fs::read_to_string(path)?;
    let instructions = parse(&code)?;
    let instructions = vm::compile(instructions)?;
    ui::run(instructions);
    Ok(())
}

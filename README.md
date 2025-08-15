# Parrot

Script and playback of text with syntax highlighting.

## Example

Create a `example.echo` file and add the following code:
```rust
// example.echo
extension "rs"
speed 100

type "fn pain() {
}"

typenl '    println!("hello world");'

goto -1 0

replace "pain" "main"
```
Then run it with:
```bash
$ parrot example.echo
```

## Syntax

To add syntax highlighting for a language currently not included:
Copy the directory into your equivalent of `~/.config/parrot/syntax/<lang>`.

### Example

To add support for `TOML` copy the `TOML` directory from this repository into `~/.config/parrot/syntax/TOML`.

## Commands

## Load

Load a file into memory

Syntax: `load <filepath> as <ident>`

## Delete

Delete selected region

Syntax: `delete`

## Goto

Move the cursor to a marker if a marker named is given, or to a position
relative to the current cursor. The position is given as `row` then `col`.

Syntax: `goto <marker>|<row> <col>`

## Insert

Insert either a string or content from memory.

Syntax: `insert <marker>|<string>` or `insert <string>`

## Select

Select the text from the current cursor position to a marker if a marker named is given, or to a relative position.
The position is given as `width` and `height`.

Syntax: `select <marker>|<width> <height>`
            
## Type

Type out the given text in the editor.

Syntax: `type <ident>|<string>`

## TypeNl

Type the given text in the editor, unlike the `type` command this will insert a
newline character and move the cursor into the new empty line and start the
typing.
This has a more natural appearance when inserting new lines into existing code.

Syntax: `typenl <ident>|<string>`
or optionally to remove the final trailing newline character:
Syntax: `typenl <ident>|<string> nonl`

## Wait / Sleep

Wait N seconds before loading the next command.
`sleep` is an alias for `wait`

Syntax: `wait <seconds>`

## Speed

Set the speed for which commands are executed / content is typed

Syntax: `speed <milliseconds>`
Default: `20`

## Line pause

Set the speed for which to wait after each newline char is typed

Syntax: `linepause <milliseconds>`
Default: `0`
            
## Replace

Selects, deletes and replaces the text.

Syntax: `replace <string> <ident>|<string>`

## Numbers

Show / hide line numbers

Syntax: `numbers <true|false>`
Default: `false`

## Clear

Clear the screen

Syntax: `clear`

## Extension

Set the file extension for the syntax highlighter

Syntax: `extension "rs"`
Default: `"txt"`

## Jitter

Pad the frame time with some jitter, making for a more natural appearance of
typing.

Syntax: `jitter 25`

## Theme

Set the theme.
To see a list of themes run `parrot --themes`.

Syntax: `theme <string>`

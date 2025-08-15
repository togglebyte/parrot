# Parrot

Script and playback of text with syntax highlighting.

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

## Line pause

Set the speed for which to wait after each newline char is typed

Syntax: `linepause <milliseconds>`
            
## Replace

Selects, deletes and replaces the text.

Syntax: `replace <string> <ident>|<string>`

## Numbers

Show / hide line numbers

Syntax: `numbers <true|false>`

## Clear

Clear the screen

Syntax: `clear`

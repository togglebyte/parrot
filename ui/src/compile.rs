use std::time::Duration;

use anathema::geometry::Size;
use parser::{Dest, Source};
use unicode_width::UnicodeWidthStr;

pub use crate::context::Context;
use crate::error::{Error, Result};
use crate::instructions::Instruction;

pub fn compile(parsed_instructions: parser::Instructions) -> Result<Vec<Instruction>> {
    let mut context = Context::new();
    let mut instructions = vec![];

    for inst in parsed_instructions {
        match inst {
            parser::Instruction::Load(path, key) => {
                let content = std::fs::read_to_string(&path).map_err(|_| Error::Import(path))?;
                context.set(key, content);
            }
            parser::Instruction::Find(needle) => instructions.push(Instruction::FindInCurrentLine(needle)),
            parser::Instruction::Goto(dest) => {
                let inst = match dest {
                    Dest::Relative { row, col } => Instruction::Jump((col, row).into()),
                    Dest::Marker(name) => Instruction::JumpToMarker(name),
                };
                instructions.push(inst);
            }
            parser::Instruction::Select { width, height } => {
                instructions.push(Instruction::Select(Size::new(width, height)))
            }
            parser::Instruction::Delete => instructions.push(Instruction::Delete),
            parser::Instruction::Type {
                source,
                trim_trailing_newline,
                prefix_newline,
            } => {
                let mut content = match source {
                    Source::Str(content) => content,
                    Source::Ident(key) => context.load(key)?,
                };

                if trim_trailing_newline && content.ends_with('\n') {
                    _ = content.pop();
                }

                if prefix_newline {
                    instructions.push(Instruction::Insert("\n".into()));
                }
                instructions.push(Instruction::LoadTypeBuffer(content));
            }
            parser::Instruction::Insert(source) => {
                let inst = match source {
                    Source::Str(content) => Instruction::Insert(content),
                    Source::Ident(key) => {
                        let content = context.load(key)?;
                        Instruction::Insert(content)
                    }
                };
                instructions.push(inst);
            }
            parser::Instruction::Replace { src, replacement } => {
                let width = src.width() as u16;
                instructions.push(Instruction::FindInCurrentLine(src));
                instructions.push(Instruction::Select(Size::new(width, 1)));
                instructions.push(Instruction::Delete);
                let inst = match replacement {
                    Source::Str(content) => Instruction::LoadTypeBuffer(content),
                    Source::Ident(key) => {
                        let content = context.load(key).unwrap();
                        Instruction::LoadTypeBuffer(content)
                    }
                };
                instructions.push(inst);
            }
            parser::Instruction::Wait(seconds) => instructions.push(Instruction::Wait(Duration::from_secs(seconds))),
            parser::Instruction::Speed(millis) => instructions.push(Instruction::Speed(Duration::from_millis(millis))),
            parser::Instruction::LinePause(millis) => {
                instructions.push(Instruction::LinePause(Duration::from_millis(millis)))
            }
            parser::Instruction::SetTitle(title) => instructions.push(Instruction::SetTitle(title)),
            parser::Instruction::SetExtension(ext) => instructions.push(Instruction::SetExtension(ext)),
            parser::Instruction::ShowLineNumbers(show) => instructions.push(Instruction::ShowLineNumbers(show)),
            parser::Instruction::Jitter(jitter) => instructions.push(Instruction::SetJitter(jitter)),
            parser::Instruction::SetTheme(theme) => instructions.push(Instruction::SetTheme(theme)),
            parser::Instruction::LoadAudio(path) => instructions.push(Instruction::LoadAudio(path)),
            parser::Instruction::Clear => instructions.push(Instruction::Clear),
            parser::Instruction::Popup(msg) => instructions.push(Instruction::Popup(msg)),
            parser::Instruction::ClosePopup => instructions.push(Instruction::ClosePopup),
        }
    }

    Ok(instructions)
}

#[cfg(test)]
mod test {}

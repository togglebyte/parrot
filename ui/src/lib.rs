use std::time::Duration;

use anathema::prelude::*;
pub use compile::compile;

use crate::editor::Editor;
use crate::instructions::Instruction;
use crate::random::Random;

mod compile;
mod context;
mod document;
mod editor;
mod error;
mod instructions;
mod markers;
mod random;
pub(crate) mod syntax;
mod textbuffer;

pub fn run(instructions: Vec<Instruction>) {
    let editor = Editor::new(instructions, Duration::from_millis(70));

    let doc = Document::new("@index");

    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();
    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);

    let template_root = dirs::config_dir().unwrap().join("parrot").join("templates");

    builder
        .component("index", template_root.join("index.aml"), editor, Default::default())
        .unwrap();
    builder.template("status", template_root.join("status.aml")).unwrap();
    builder.template("error", template_root.join("error.aml")).unwrap();
    let res = builder.finish(&mut backend, |runtime, backend| runtime.run(backend));

    match res {
        Ok(()) | Err(anathema::runtime::Error::Stop) => {}
        Err(e) => eprintln!("{e}"),
    }
}

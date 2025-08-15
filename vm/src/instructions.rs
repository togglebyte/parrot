use std::time::Duration;

use anathema::geometry::{Pos, Size};

#[derive(Debug)]
pub enum Instruction {
    // Relative jump
    Jump(Pos),
    JumpToMarker(String),
    Select(Size),

    // -----------------------------------------------------------------------------
    //   - Modifying instructions -
    // -----------------------------------------------------------------------------
    // * Require new highlighting
    // * If the `content` contains a newline then offset all the subsequent markers
    LoadTypeBuffer(String),
    // Inserts all the content at once, unlike Type which types the content out
    // character by character
    Insert(String),
    // Remove all character in the highlighted range of the editor, or 
    // if no selection exists: remove the character under the cursor
    Delete,
    Wait(Duration),
    Speed(Duration),
    LinePause(Duration),

    FindInCurrentLine(String),

    SetTitle(String),
    ShowLineNumbers(bool),
    Clear,
}

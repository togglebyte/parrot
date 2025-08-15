use std::ops::Range;

use anathema::geometry::{Pos, Region};
use unicode_width::UnicodeWidthChar;

use crate::markers::{Marker, Markers};

// NOTE:
// Document ONLY needs to regenerate the markers when
// adding text that contains one or more markers!

#[derive(Debug)]
pub(crate) struct Document {
    pub markers: Markers,
    text: String,
}

impl Document {
    pub fn new(text: impl Into<String>) -> Self {
        let (text, markers) = crate::markers::generate(text);
        let markers = markers.unwrap_or_else(Markers::new);
        Self { text, markers }
    }

    pub fn add_markers(&mut self, row: i32, markers: Markers) {
        self.markers.merge(row as usize, markers);
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn lookup_marker(&self, key: &str) -> Option<&Marker> {
        self.markers.get(key)
    }

    fn byte_offset(&self, pos: Pos) -> usize {
        let line_offset = self.text.split_inclusive('\n').map(str::len).take(pos.y as usize).sum();
        let Some(line) = self.text[line_offset..].split('\n').next() else { return line_offset };

        if pos.x == 0 {
            return line_offset;
        }

        let mut x = 0;
        for (i, c) in line.char_indices() {
            x += c.width().unwrap_or(0);

            if x as i32 >= pos.x {
                return line_offset + i + c.len_utf8();
            }
        }

        line_offset + line.len()
    }

    pub fn insert_str(&mut self, pos: Pos, s: impl AsRef<str>) {
        let s = s.as_ref();
        let index = self.byte_offset(pos);
        self.text.insert_str(index, s);

        // If the string contains a newline character then offset all the markers by one
        let newlines = s.chars().filter(|c| *c == '\n').count();
        if newlines > 0 {
            self.markers.offset_after(pos.y as usize, newlines);
        }
    }

    // Get the byte position in the string
    pub(crate) fn get_byte_offset(&self, pos: Pos, mut width: usize) -> Range<usize> {
        let start = self.byte_offset(pos);
        let line = &self.text[start..];

        let mut end = start;
        for (i, c) in line.char_indices() {
            if c == '\n' {
                end = start + i;
                break;
            }

            width = width.saturating_sub(c.width().unwrap_or(0));

            if width == 0 {
                end = start + i;
                break;
            }
        }

        start..end
    }

    pub(crate) fn delete(&mut self, region: Region) {
        for y in region.from.y..region.to.y {
            let pos = Pos::new(region.from.x, y);
            let width = 1 + region.to.x - region.from.x;
            _ = self.text.drain(self.get_byte_offset(pos, width as usize));
        }
        // _ = self.text.drain(self.get_byte_offset(pos, width));
    }

    pub(crate) fn find(&self, cursor: Pos, needle: String) -> Option<usize> {
        let (_, y) = (cursor.x, cursor.y);
        let line_offset = self.byte_offset(Pos::new(0, y));
        let text = &self.text[line_offset..];

        let end = text.bytes().take_while(|b| *b != b'\n').count();
        let line = &text[..end];

        line.find(&needle)
    }

    pub(crate) fn clear(&mut self) {
        self.markers.clear();
        self.text.clear();
    }
}

#[cfg(test)]
mod test {
    use anathema::geometry::Size;

    use super::*;

    #[test]
    fn delete_region() {
        let text = "abcdefg
1234567
abcdefg
1234567
abcdefg";
        let mut doc = Document::new(text);

        let region = Region::from((Pos::new(1, 1), Size::new(2, 3)));
        doc.delete(region);

        let expected = "abcdefg
14567
adefg
14567
abcdefg";

        let actual = doc.text();
        assert_eq!(expected, actual);
    }

    #[test]
    fn insert_offsets_marker() {
        static NEWLINES: usize = 4;
        let text = "// @zero
hello
// @one
world
// @two
!
";
        let mut doc = Document::new(text);

        eprintln!("{:#?}", &doc.markers);

        let row = doc.lookup_marker("one").map(|m| m.row as i32).unwrap();
        doc.insert_str(Pos::new(0, row), "\n".repeat(NEWLINES));

        eprintln!("{}", doc.text());
        eprintln!("{:#?}", &doc.markers);

        let zero = doc.lookup_marker("zero").map(|m| m.row as i32).unwrap();
        let one = doc.lookup_marker("one").map(|m| m.row as i32).unwrap() as usize;
        let two = doc.lookup_marker("two").map(|m| m.row as i32).unwrap() as usize;

        assert_eq!(zero, 0);
        assert_eq!(one, 1 + NEWLINES);
        assert_eq!(two, 2 + NEWLINES);
    }
}

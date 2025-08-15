use anathema::state::Color;
use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, Style, Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

// -----------------------------------------------------------------------------
//   - Scratch buffer -
// -----------------------------------------------------------------------------
pub struct InactiveScratch {
    lines: Lines<'static>,
}

impl InactiveScratch {
    pub unsafe fn activate<'a>(&mut self, code: &'a str) -> ActiveScratch<'a> {
        ActiveScratch {
            lines: unsafe { std::mem::transmute(&mut self.lines) },
            code,
        }
    }

    pub(crate) fn new() -> Self {
        Self { lines: Lines::new() }
    }
}

pub struct ActiveScratch<'a> {
    lines: &'a mut Lines<'a>,
    code: &'a str,
}

impl ActiveScratch<'_> {
    pub fn with<F>(self, mut f: F)
    where
        for<'a> F: FnMut(&mut Lines<'a>, &'a str),
    {
        f(self.lines, self.code);
    }
}

impl Drop for ActiveScratch<'_> {
    fn drop(&mut self) {
        self.lines.reset();
    }
}

// -----------------------------------------------------------------------------
//   - Lines buffer -
// -----------------------------------------------------------------------------
#[derive(Debug, Default)]
pub struct Lines<'a> {
    spans: Vec<Span<'a>>,
    lines: Vec<usize>,
}

impl<'a> Lines<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    fn reset(&mut self) {
        self.spans.clear();
        self.lines.clear();
    }

    fn newline(&mut self) {
        self.lines.push(self.spans.len());
    }

    fn push_span(&mut self, span: Span<'a>) {
        self.spans.push(span);
    }

    pub fn iter(&self) -> impl Iterator<Item = &[Span<'_>]> {
        let mut index = 0;
        let mut prev = 0;
        std::iter::from_fn(move || {
            if index == self.lines.len() {
                return None;
            }

            let range = prev..self.lines[index];
            prev = self.lines[index];
            index += 1;
            let spans = &self.spans[range];
            Some(spans)
        })
    }
}

// -----------------------------------------------------------------------------
//   - Span -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Span<'a> {
    pub style: Style,
    pub src: &'a str,
}

impl Span<'_> {
    pub(crate) fn style(&self) -> anathema::widgets::Style {
        let mut style = anathema::widgets::Style::new();

        let fg = self.style.foreground;
        style.fg = Some(Color::Rgb(fg.r, fg.g, fg.b));
        style.set_bold(self.style.font_style.contains(FontStyle::BOLD));
        style.set_italic(self.style.font_style.contains(FontStyle::ITALIC));

        style
    }
}

// -----------------------------------------------------------------------------
//   - Highligher -
// -----------------------------------------------------------------------------
pub struct Highlighter {
    ps: SyntaxSet,
    theme: Theme,
}

impl Highlighter {
    pub fn new() -> Self {
        #[cfg(not(debug_assertions))]
        let ps = SyntaxSet::load_defaults_newlines();
        #[cfg(debug_assertions)]
        let ps = SyntaxSet::new();
        let mut builder = ps.into_builder();

        // Try to load syntaxes from a config dir
        let root = dirs::config_dir().unwrap().join("parrot");
        let syntaxes = root.join("syntax");
        _ = builder.add_from_folder(syntaxes, true);

        builder.add_plain_text_syntax();
        let ps = builder.build();

        // Set the theme
        // TODO: fall back to another theme.
        // Maybe add an actual config file where this can be specified?
        let theme_path = root.join("theme");
        let theme = ThemeSet::get_theme(theme_path).expect("missing theme");

        Self { ps, theme }
    }

    pub fn highlight<'a>(&self, src: &'a str, ext: &str, buffer: &mut Lines<'a>) {
        buffer.reset();

        let syntax = self
            .ps
            .find_syntax_by_extension(ext)
            .unwrap_or_else(|| self.ps.find_syntax_plain_text());

        let mut h = HighlightLines::new(syntax, &self.theme);

        for line in LinesWithEndings::from(src) {
            // LinesWithEndings enables use of newlines mode
            let spans = h.highlight_line(line, &self.ps).unwrap();
            for (style, src) in spans {
                buffer.push_span(Span { style, src });
            }
            buffer.newline();
        }
    }
}

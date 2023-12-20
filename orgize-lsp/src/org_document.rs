use orgize::{export::Traverser, Org};
use std::iter::once;
use tower_lsp::lsp_types::{Position, Range};

pub struct OrgDocument {
    pub text: String,
    pub line_starts: Vec<u32>,
    pub org: Org,
}

impl OrgDocument {
    pub fn new(text: impl AsRef<str>) -> Self {
        let text = text.as_ref().to_string();

        OrgDocument {
            org: Org::parse(&text),
            line_starts: line_starts(&text),
            text,
        }
    }

    pub fn update(&mut self, start: u32, end: u32, text: &str) {
        self.text
            .replace_range((start as usize)..(end as usize), text);

        self.line_starts = line_starts(&self.text);

        self.org = Org::parse(&self.text);
    }

    pub fn position_of(&self, offset: u32) -> Position {
        let line = self
            .line_starts
            .binary_search(&offset)
            .unwrap_or_else(|i| i - 1);

        let line_start = self.line_starts[line];

        let character = self.text.as_str()[(line_start as usize)..(offset as usize)]
            .chars()
            .count();

        Position::new(line as u32, character as u32)
    }

    pub fn line_of(&self, offset: u32) -> u32 {
        self.line_starts
            .binary_search(&offset)
            .unwrap_or_else(|i| i - 1) as u32
    }

    pub fn range_of(&self, start_offset: u32, end_offset: u32) -> Range {
        Range::new(self.position_of(start_offset), self.position_of(end_offset))
    }

    pub fn offset_of(&self, position: Position) -> u32 {
        let line_start = self.line_starts[position.line as usize] as usize;

        let index = self.text.as_str()[line_start..]
            .char_indices()
            .nth(position.character as usize)
            .map(|(i, _)| i)
            .unwrap_or_default();

        (line_start + index) as u32
    }

    pub fn traverse<H: Traverser>(&self, h: &mut H) {
        self.org.traverse(h);
    }
}

fn line_starts(text: &str) -> Vec<u32> {
    let bytes = text.as_bytes();

    once(0)
        .chain(
            memchr::memchr2_iter(b'\r', b'\n', bytes)
                .filter(|&i| bytes[i] == b'\n' || !matches!(bytes.get(i + 1), Some(b'\n')))
                .map(|i| (i + 1) as u32),
        )
        .collect()
}

#[test]
fn test() {
    let doc = OrgDocument::new(
        r#"* toc :toc:

fsfs
fasdfs



fasdfs
 
*a* _a_ /1/ ~default~ =default= a_a

# abc

* abc12121
12121


#+begin_src javascript
console.log(a);
#+end_src

"#,
    );

    let start = 12;
    let start_position = Position {
        line: 1,
        character: 0,
    };
    let end = 81;
    let end_position = Position {
        line: 13,
        character: 0,
    };

    assert_eq!(doc.position_of(start), start_position);
    assert_eq!(doc.position_of(end), end_position);

    assert_eq!(doc.offset_of(start_position), start);
    assert_eq!(doc.offset_of(end_position), end);
}

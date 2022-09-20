use tree_sitter::{Language, LanguageError, Parser, Query, QueryCursor, TextProvider};

extern "C" {
    fn tree_sitter_rust() -> Language;
}

pub struct Finder {
    parser: Parser,
    query: Query,
}

#[derive(Debug, PartialEq)]
pub struct Finding {
    start: usize,
    len: usize,
}

impl Finding {
    #[cfg(test)]
    pub fn new(start: usize, len: usize) -> Finding {
        Finding { start, len }
    }

    pub fn lookup<'a>(&self, buf: &'a [u8]) -> &'a str {
        std::str::from_utf8(&buf[self.start..self.start + self.len]).unwrap()
    }

    pub fn line(&self, buf: &[u8]) -> usize {
        buf[..self.start]
            .as_ref()
            .iter()
            .filter(|ch| **ch == b'\n')
            .count()
    }

    pub fn is_side_effect(&self, buf: &[u8]) -> bool {
        if buf.get(self.start + self.len) == Some(&b';') {
            for ch in buf[..self.start]
                .as_ref()
                .iter()
                .rev()
                .take_while(|ch| **ch != b'\n')
            {
                if *ch != b' ' && *ch != b'\t' {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }

    pub fn cut_range(&self, buf: &[u8]) -> std::ops::Range<usize> {
        if self.is_side_effect(buf) {
            let mut start = self.start;
            while start > 0 {
                if buf[start] == b'\n' {
                    break;
                }
                start -= 1;
            }

            let mut end = self.start + self.len;
            while end < buf.len() {
                if buf[end] == b'\n' {
                    break;
                }
                end += 1;
            }

            start..end
        } else {
            self.start..self.start + self.len
        }
    }

    pub fn preserve_range(&self, buf: &[u8]) -> Option<std::ops::Range<usize>> {
        if self.is_side_effect(buf) {
            None
        } else {
            Some(self.start + "dbg!(".len()..self.start + self.len - 1)
        }
    }
}

impl Finder {
    pub fn new() -> Result<Finder, LanguageError> {
        let mut parser = Parser::new();
        let lang = unsafe { tree_sitter_rust() };

        parser.set_language(lang)?;
        let query = Query::new(
            lang,
            "(macro_invocation macro: (identifier) @name (token_tree) @token (#eq? @name \"dbg\")) @macro",
        )
        .unwrap();

        Ok(Finder { parser, query })
    }

    pub fn find(&mut self, buf: &[u8]) -> Vec<Finding> {
        let mut res = Vec::new();
        let mut cursor = QueryCursor::new();
        let tree = self.parser.parse(&buf, None).unwrap();

        for capt in cursor.matches(&self.query, tree.root_node(), buf) {
            for capt in capt.nodes_for_capture_index(2) {
                let mut text = buf[..].as_ref().text(capt);
                if let Some(content) = text.next() {
                    let start = content.as_ptr() as usize - buf.as_ptr() as usize;
                    let len = content.len();

                    res.push(Finding { start, len });
                } else {
                    panic!("issue");
                }
            }
        }

        res
    }
}

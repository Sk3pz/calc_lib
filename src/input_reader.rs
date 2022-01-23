use crate::InputPos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InputReader {
    stream: Vec<char>,
    pos: InputPos,
}

impl InputReader {
    pub(crate) fn new<S: Into<String>>(input: S) -> Self {
        Self {
            stream: input.into().chars().collect(),
            pos: InputPos::default(),
        }
    }

    pub(crate) fn pos(&self) -> InputPos {
        self.pos.clone()
    }

    pub(crate) fn consume(&mut self) -> Option<char> {
        if self.stream.is_empty() {
            return None;
        }

        let ch = self.stream.remove(0);
        self.pos.next();

        if ch == '\n' {
            self.pos.newline();
        }

        Some(ch)
    }

    pub(crate) fn peek_at(&self, n: usize) -> Option<char> {
        self.stream.get(n).cloned()
    }

    pub(crate) fn peek(&self) -> Option<char> {
        self.peek_at(0)
    }

    pub(crate) fn is_empty(&self) -> bool { self.stream.is_empty() }
}
pub struct Scanner {
    text: String,
}

impl Scanner {
    pub fn new(text: String) -> Scanner{
        Scanner {
            text,
        }
    }
}

impl Scanner {
    pub fn is_empty(&mut self) -> bool {
        self.text.is_empty()
    }

    pub fn is_next_token(&mut self, text: &str) -> bool {
        self.text.starts_with(&text)
    }

    pub fn does_next_match<F>(&mut self, predicate: F) -> bool
    where F: Fn(&char) -> bool {
        let mut chars = self.text.chars();
        let next_ch = chars.next().unwrap();
        predicate(&next_ch)
    }

    pub fn take_while<F>(&mut self, predicate: F) -> String
    where F: Fn(&char) -> bool {
        let chars = self.text.chars();
        let value = chars.take_while(predicate).collect();
        self.text = self.text.strip_prefix(&value).unwrap().to_string();
        value
    }

    pub fn take_token(&mut self, token: &str) {
        self.text = self.text.strip_prefix(&token).unwrap().to_string();
    }

    pub fn strip_whitespace(&mut self) {
        self.take_while(|ch| ch.is_whitespace());
    }
}
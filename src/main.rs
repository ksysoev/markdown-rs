mod markdown {

    pub struct Scanner<'a> {
        input: std::str::Chars<'a>,
        buffer: Vec<char>,
    }

    impl<'a> Scanner<'a> {
        pub fn new(input: std::str::Chars<'a>) -> Scanner<'a> {
            Scanner {
                input: input,
                buffer: Vec::new(),
            }
        }

        pub fn next(&mut self) -> Option<char> {
            if self.buffer.len() > 0 {
                return Some(self.buffer.remove(0));
            }

            let c = self.input.next().unwrap_or(0 as char);

            if c == 0 as char {
                return None;
            }

            Some(c)
        }

        pub fn peek(&mut self) -> Option<char> {
            if self.buffer.len() > 0 {
                return Some(self.buffer.get(0).unwrap().clone());
            }

            let c = self.input.next().unwrap_or(0 as char);

            if c == 0 as char {
                return None;
            }

            self.buffer.push(c.clone());

            Some(c)
        }
    }

    impl<'a> Iterator for Scanner<'a> {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            self.next()
        }
    }
}

fn main() {
    let input = "# T1\n## T2\n### T3\n";

    let iter = input.chars();

    let mut scanner = markdown::Scanner::new(iter);
    // let mut tokenizer = markdown::Tokenizer::new(scanner);

    while let Some(c) = scanner.next() {
        print!("{}", c);
        print!("{}", scanner.peek().unwrap_or(' '));
    }
}

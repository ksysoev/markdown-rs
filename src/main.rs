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

    #[derive(Debug)]
    pub enum Token {
        Heading(u8),
        Text(String),
        Newline,
    }

    pub fn tokenize(input: std::str::Chars) -> Vec<Token> {
        let mut scanner = Scanner::new(input);

        let mut tokens = Vec::new();

        while let Some(c) = scanner.next() {
            match c {
                '#' => {
                    let mut level = 1;
                    while scanner.peek() == Some('#') {
                        scanner.next();
                        level += 1;
                    }

                    tokens.push(Token::Heading(level));
                }
                '\n' => {
                    tokens.push(Token::Newline);
                }
                _ => {
                    let mut text = String::new();
                    text.push(c);

                    while let Some(c) = scanner.peek() {
                        match c {
                            '\n' => break,
                            _ => {
                                text.push(c);
                                scanner.next();
                            }
                        }
                    }

                    tokens.push(Token::Text(text));
                }
            }
        }

        tokens
    }
}

fn main() {
    let input = "# T1\n## T2\n### T3\n";

    let tokens = markdown::tokenize(input.chars());

    for token in tokens {
        println!("{:?}", token);
    }
}

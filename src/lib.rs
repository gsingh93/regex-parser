#![feature(collections)]

pub mod parser {
    pub type ParseResult = Result<RegEx, &'static str>;

    #[derive(Eq, PartialEq, Debug)]
    pub enum RegEx {
        Or(Box<RegEx>, Box<RegEx>),
        Sequence(Vec<Box<RegEx>>),
        Repetition(Box<RegEx>),
        Terminal(char),
    }

    pub struct RegExParser {
        s: String,
        pos: usize
    }

    impl RegExParser {
        pub fn new(s: String) -> RegExParser {
            RegExParser { s: s, pos: 0 }
        }

        pub fn parse(&mut self) -> ParseResult {
            self.regex()
        }

        fn regex(&mut self) -> ParseResult {
            let t1 = try!(self.term());
            if self.more() && self.peek() == '|' {
                self.consume('|').unwrap();
                let t2 = try!(self.term());
                Ok(RegEx::Or(Box::new(t1), Box::new(t2)))
            } else {
                Ok(t1)
            }
        }

        fn term(&mut self) -> ParseResult {
            let mut v = Vec::new();
            while self.more() && self.peek() != ')' && self.peek() != '|' {
                let f = try!(self.factor());
                v.push(Box::new(f));
            }
            Ok(RegEx::Sequence(v))
        }

        fn factor(&mut self) -> ParseResult {
            let b = try!(self.base());
            if self.more() && self.peek() == '*' {
                self.consume('*').unwrap();
                Ok(RegEx::Repetition(Box::new(b)))
            } else {
                Ok(b)
            }
        }

        fn base(&mut self) -> ParseResult {
            if self.peek() == '(' {
                self.consume('(').unwrap();
                let r = try!(self.regex());
                self.consume(')').unwrap();
                Ok(r)
            } else {
                Ok(RegEx::Terminal(self.next()))
            }
        }

        fn peek(&self) -> char {
            assert!(self.s.len() > self.pos);
            self.s.char_at(self.pos)
        }

        fn consume(&mut self, c: char) -> Result<(), String> {
            let p = self.peek();
            if p == c {
                self.pos += 1;
                Ok(())
            } else {
                Err(format!("Expected {} at position {} but got {}", c, self.pos, p))
            }
        }

        fn next(&mut self) -> char {
            let c = self.peek();
            self.consume(c).unwrap();
            c
        }

        fn more(&self) -> bool {
            self.s.len() > self.pos
        }
    }
}

#[cfg(test)]
mod test {
    use parser::RegEx::{Or, Repetition, Sequence, Terminal};
    use parser::RegExParser;

    #[test]
    fn test_basic() {
        let mut p = RegExParser::new("".to_string());
        assert_eq!(p.parse(), Ok(Sequence(vec![])));

        let mut p = RegExParser::new("a".to_string());
        assert_eq!(p.parse(), Ok(Sequence(vec![Box::new(Terminal('a'))])));

        let mut p = RegExParser::new("a|b".to_string());
        assert_eq!(p.parse(), Ok(Or(Box::new(Sequence(vec![Box::new(Terminal('a'))])),
                                    Box::new(Sequence(vec![Box::new(Terminal('b'))])))));

        let mut p = RegExParser::new("a*".to_string());
        assert_eq!(p.parse(), Ok(Sequence(vec![Box::new(Repetition(Box::new(Terminal('a'))))])));

        let mut p = RegExParser::new("((a|b*)|a*)|aab".to_string());
        let res = Ok(Or(
            Box::new(Sequence(vec![Box::new(Or(
                Box::new(Sequence(
                    vec![Box::new(Or(Box::new(Sequence(vec![Box::new(Terminal('a'))])),
                                     Box::new(Sequence(
                                         vec![Box::new(Repetition(Box::new(Terminal('b'))))]))))])),
                Box::new(Sequence(vec![Box::new(Repetition(Box::new(Terminal('a'))))]))))])),
            Box::new(Sequence(vec![Box::new(Terminal('a')), Box::new(Terminal('a')),
                                   Box::new(Terminal('b'))]))));
        assert_eq!(p.parse(), res);
    }
}

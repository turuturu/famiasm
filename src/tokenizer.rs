//use crate::aaa::Loc;

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Loc(pub usize, pub usize);

// impl Loc {
//     fn merge(&self, other: &Loc) {
//         use std::cmp::{max, min};
//         Loc(min(self.0, other.0), max(self.1, other.1));
//     }
// }
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Loc(pub usize, pub usize);

impl Loc {
    fn merge(&self, other: &Loc) {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    pub fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexErrorKind {
    InvalidChar(char),
    Eof,
}
type LexError = Annot<LexErrorKind>;

impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }
    fn eof(loc: Loc) -> Self {
        LexError::new(LexErrorKind::Eof, loc)
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub enum TokenKind {
    Opcode(Vec<char>),
    Adr(Vec<char>),
    Im(Vec<char>),
    Label(Vec<char>),
    Comment(Vec<char>),
    Directive(Vec<char>),
    String(Vec<char>),
    X,
    Y,
    A,
    Comma,
    LParen,
    RParen,
    Spaces,
}

// #[derive(Debug)]
// pub struct Token{
//     val: Vec<char>,
//     kind: TokenKind,
// }
type Token = Annot<TokenKind>;

impl Token {
    fn im(n: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::Im(n), loc)
    }
    fn label(label: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::Label(label), loc)
    }
    fn opcode(opcode: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::Opcode(opcode), loc)
    }
    fn comment(comment: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::Comment(comment), loc)
    }
    fn comma(loc: Loc) -> Self {
        Self::new(TokenKind::Comma, loc)
    }
    fn directive(directive: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::Directive(directive), loc)
    }
    fn string(string: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::String(string), loc)
    }
    fn adr(adr: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::Adr(adr), loc)
    }
    fn x(loc: Loc) -> Self {
        Self::new(TokenKind::X, loc)
    }
    fn y(loc: Loc) -> Self {
        Self::new(TokenKind::Y, loc)
    }
    fn a(loc: Loc) -> Self {
        Self::new(TokenKind::A, loc)
    }
    fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }
    fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
    fn spaces(loc: Loc) -> Self {
        Self::new(TokenKind::Spaces, loc)
    }
}

// use std::str::FromStr;
// impl FromStr for Token {
//     type Err = ();
//     fn from_str(s: &str) -> Result<Self, Self::Err>{
//         match s {
//             "X" | "x" => Ok(Token::X),
//             "Y" | "y" => Ok(Token::Y),
//             "(" => Ok(Token::LParen),
//             ")" => Ok(Token::RParen),
//             _ => Ok(Token::A), // <= temporary
//         }
//     }
// }

pub fn tokenize(line: impl Into<String>) -> Vec<Token> {
    let buf: &Vec<char> = &line.into().chars().collect();
    let mut pos = 0;
    let mut tokens: Vec<Token> = Vec::new();
    println!("{:?}", buf);
    let mut has_op = false;
    while pos < buf.len() {
        let mut cur = pos;
        let mut head_ch = buf[cur];
        // separator
        if head_ch == '(' || head_ch == ')' || head_ch == ',' {
            tokens.push(
                match head_ch {
                    ',' => Token::comma(Loc(pos, pos+1)),
                    '(' => Token::lparen(Loc(pos, pos+1)),
                    ')' => Token::rparen(Loc(pos, pos+1)),
                    _ => panic!(),
                },
            );
            pos = pos + 1;
            continue;
        }
        // comment
        if head_ch == ';' {
            tokens.push(Token::comment(buf[pos..buf.len()].to_vec(), Loc(pos, buf.len())));
            break;
        }
        // variables
        if head_ch.is_ascii_alphabetic() || buf[cur] == '.' {
            let is_head = cur == 0;
            while cur < buf.len()
                && (buf[cur].is_ascii_alphabetic() || buf[cur] == ':' || buf[cur] == '.')
            {
                cur += 1;
            }
            if is_head || buf[cur - 1] == ':' || has_op {
                tokens.push(Token::label(buf[pos..cur].to_vec(), Loc(pos, cur)));
            } else if cur - pos == 1
                && (buf[cur - 1] == 'X'
                    || buf[cur - 1] == 'x'
                    || buf[cur - 1] == 'Y'
                    || buf[cur - 1] == 'y'
                    || buf[cur - 1] == 'a'
                    || buf[cur - 1] == 'A')
            {
                tokens.push(
                    match buf[cur - 1] {
                        'X' | 'x' => Token::x(Loc(pos, cur)),
                        'Y' | 'y' => Token::y(Loc(pos, cur)),
                        'A' | 'a' => Token::a(Loc(pos, cur)),
                        _ => panic!(""),
                    },
                );
            } else if buf[pos] == '.' {
                tokens.push(Token::directive(buf[pos..cur].to_vec(), Loc(pos, cur)));
            } else {
                has_op = true;
                tokens.push(Token::opcode(buf[pos..cur].to_vec(), Loc(pos, cur)));
            }
            pos = cur;
            continue;
        }
        // spaces
        if head_ch == ' ' || head_ch == '\t' {
            while cur < buf.len() && (buf[cur] == ' ' || buf[cur] == '\t') {
                cur += 1;
            }
            tokens.push(Token::spaces(Loc(pos, cur)));
            pos = cur;
            continue;
        }
        // string
        if head_ch == '"' {
            cur += 1;
            while cur < buf.len() && buf[cur] != '"' {
                cur += 1;
            }
            if buf[cur] == '"' {
                cur += 1;
            } else {
                panic!("\" not found");
            }
            tokens.push(Token::string(buf[pos..cur].to_vec(), Loc(pos, cur)));
            pos = cur;
            continue;
        }

        // number
        if head_ch == '#' {
            cur += 1;
            head_ch = buf[cur];
        }
        if head_ch == '$' {
            cur += 1;
            while cur < buf.len() && buf[cur].is_digit(16) {
                cur += 1;
            }
            tokens.push(
                if buf[pos] == '#' {
                    Token::adr(buf[pos..cur].to_vec(), Loc(pos, cur))
                } else {
                    Token::im(buf[pos..cur].to_vec(), Loc(pos, cur))
                },
            );
            pos = cur + 1;
            continue;
            //assert!((cur - pos) == 3 || (cur - pos) == 5);
        } else if head_ch == '%' {
            cur += 1;
            while cur < buf.len() && buf[cur].is_digit(2) {
                cur += 1;
            }
            tokens.push(
                if buf[pos] == '#' {
                    Token::adr(buf[pos..cur].to_vec(), Loc(pos, cur))
                } else {
                    Token::im(buf[pos..cur].to_vec(), Loc(pos, cur))
                },
            );
            pos = cur + 1;
            continue;
            //assert!(((cur - pos) == 5 || (cur - pos) == 9));
        } else if head_ch.is_digit(10) {
            while cur < buf.len() && buf[cur].is_digit(10) {
                cur += 1;
            }
            tokens.push(
                if buf[pos] == '#' {
                    Token::adr(buf[pos..cur].to_vec(), Loc(pos, cur))
                } else {
                    Token::im(buf[pos..cur].to_vec(), Loc(pos, cur))
                },
            );
            pos = cur + 1;
            continue;
        }
        println!("{:?}", buf);
        panic!();
    }
    tokens
}

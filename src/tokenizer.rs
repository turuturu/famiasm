use crate::common::{Annot, Loc};
use log::debug;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexErrorKind {
    #[allow(dead_code)]
    InvalidChar(char),
    #[allow(dead_code)]
    Eof,
}
#[allow(dead_code)]
type LexError = Annot<LexErrorKind>;

impl LexError {
    #[allow(dead_code)]
    fn invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }
    #[allow(dead_code)]
    fn eof(loc: Loc) -> Self {
        LexError::new(LexErrorKind::Eof, loc)
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub enum TokenKind {
    Opcode(Vec<char>),
    Adr8(u8),
    Adr16(u16),
    U8(u8),
    U16(u16),
    Im(u8),
    LabelDef(Vec<char>),
    Label(Vec<char>),
    Comment(Vec<char>),
    Directive(Vec<char>),
    String(Vec<char>),
    X,
    Y,
    A,
    #[allow(dead_code)]
    Comma,
    LParen,
    RParen,
    #[allow(dead_code)]
    Spaces,
    Arrow,
}

pub type Token = Annot<TokenKind>;
impl Token {
    fn im(n: u8, loc: Loc) -> Self {
        Self::new(TokenKind::Im(n), loc)
    }
    fn label(label: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::Label(label), loc)
    }
    fn label_def(label: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::LabelDef(label), loc)
    }
    fn opcode(opcode: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::Opcode(opcode), loc)
    }
    #[allow(dead_code)]
    fn comment(comment: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::Comment(comment), loc)
    }
    #[allow(dead_code)]
    fn comma(loc: Loc) -> Self {
        Self::new(TokenKind::Comma, loc)
    }
    fn directive(directive: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::Directive(directive), loc)
    }
    fn string(string: Vec<char>, loc: Loc) -> Self {
        Self::new(TokenKind::String(string), loc)
    }
    fn adr8(adr: u8, loc: Loc) -> Self {
        Self::new(TokenKind::Adr8(adr), loc)
    }
    fn adr16(adr: u16, loc: Loc) -> Self {
        Self::new(TokenKind::Adr16(adr), loc)
    }
    fn u8(adr: u8, loc: Loc) -> Self {
        Self::new(TokenKind::U8(adr), loc)
    }
    fn u16(adr: u16, loc: Loc) -> Self {
        Self::new(TokenKind::U16(adr), loc)
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
    #[allow(dead_code)]
    fn spaces(loc: Loc) -> Self {
        Self::new(TokenKind::Spaces, loc)
    }
    fn arrow(loc: Loc) -> Self {
        Self::new(TokenKind::Arrow, loc)
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
    debug!("{:?}", buf);
    let mut has_op = false;
    while pos < buf.len() {
        let mut cur = pos;
        let mut head_ch = buf[cur];
        // separator
        if head_ch == ',' {
            // skip comma token
            //tokens.push(Token::comma(Loc(pos, pos+1)));
            pos = pos + 1;
            continue;
        }
        if head_ch == '(' || head_ch == ')' || head_ch == '<' {
            tokens.push(match head_ch {
                '(' => Token::lparen(Loc(pos, pos + 1)),
                ')' => Token::rparen(Loc(pos, pos + 1)),
                '<' => Token::arrow(Loc(pos, pos + 1)),
                _ => panic!(),
            });
            pos = pos + 1;
            continue;
        }
        // comment
        if head_ch == ';' {
            //tokens.push(Token::comment(buf[pos..buf.len()].to_vec(), Loc(pos, buf.len())));
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
            debug!("{:?}, {:?}, {:?}, ", head_ch, cur, pos);
            if is_head || buf[cur - 1] == ':' {
                tokens.push(Token::label_def(buf[pos..cur].to_vec(), Loc(pos, cur)));
            } else if cur - pos == 1
                && (buf[cur - 1] == 'X'
                    || buf[cur - 1] == 'x'
                    || buf[cur - 1] == 'Y'
                    || buf[cur - 1] == 'y'
                    || buf[cur - 1] == 'a'
                    || buf[cur - 1] == 'A')
            {
                tokens.push(match buf[cur - 1] {
                    'X' | 'x' => Token::x(Loc(pos, cur)),
                    'Y' | 'y' => Token::y(Loc(pos, cur)),
                    'A' | 'a' => Token::a(Loc(pos, cur)),
                    _ => panic!(""),
                });
            } else if is_head || buf[cur - 1] == ':' || has_op {
                tokens.push(Token::label(buf[pos..cur].to_vec(), Loc(pos, cur)));
            } else if buf[pos] == '.' {
                has_op = true;
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
            // skip spaces token
            // tokens.push(Token::spaces(Loc(pos, cur)));
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
            tokens.push(Token::string(buf[pos + 1..cur - 1].to_vec(), Loc(pos, cur)));
            pos = cur;
            continue;
        }

        // number
        if head_ch == '#' {
            cur += 1;
            head_ch = buf[cur];
        }
        let mut start_pos = cur;
        let radix = match head_ch {
            '$' => {
                start_pos += 1;
                cur += 1;
                16
            }
            '%' => {
                start_pos += 1;
                cur += 1;
                2
            }
            '0'..='9' => 10,
            _ => panic!(),
        };
        while cur < buf.len() && buf[cur].is_digit(radix) {
            cur += 1;
        }
        let str: String = buf[start_pos..cur].into_iter().collect();
        tokens.push(if buf[pos] == '#' {
            Token::im(u8::from_str_radix(&str, radix).unwrap(), Loc(pos, cur))
        } else {
            let val = u16::from_str_radix(&str, radix).unwrap();
            // todo
            // 多分下記条件曖昧
            // u16で256未満の場合等
            // ディレクティブの種類で判別
            if (radix == 2 && cur - start_pos == 8)
                || (radix == 16 && cur - start_pos == 2)
                || (radix == 10 && val <= 256)
            {
                if let TokenKind::Directive(_) = &tokens.last().unwrap().value {
                    Token::u8(u8::from_str_radix(&str, radix).unwrap(), Loc(pos, cur))
                } else {
                    Token::adr8(u8::from_str_radix(&str, radix).unwrap(), Loc(pos, cur))
                }
            } else {
                if let TokenKind::Directive(_) = &tokens.last().unwrap().value {
                    Token::u16(val, Loc(pos, cur))
                } else {
                    Token::adr16(val, Loc(pos, cur))
                }
            }
            //Token::adr(u8::from_str_radix(&str,radix).unwrap(), Loc(pos, cur))
        });
        pos = cur + 1;
        continue;
    }
    tokens
}

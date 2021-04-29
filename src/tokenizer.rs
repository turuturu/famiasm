// pub enum TokenKind{
//     Op(Opcode),
//     Im8(u8),
//     Im8Adr(u8),
//     Im16(u16),
//     X,
//     Y,
//     A,
//     Comma,
//     LParen,
//     RParen,
// }
#[derive(PartialOrd, PartialEq, Debug)]
pub enum TokenKind{
    Opcode,
    Adr,
    Im,
    X,
    Y,
    A,
    Comma,
    LParen,
    RParen,
    Label,
    Comment,
    Directive,
    String,
    Spaces,
    EOL,
}

#[derive(Debug)]
pub struct Token{
    val: Vec<char>,
    kind: TokenKind,
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

pub struct Tokenizer {
    buf: Vec<char>,
    pos: usize,
    tokens: Vec<Token>,
}

impl Tokenizer {
    // pub fn new(line: impl Into<String>) -> Tokenizer {
    //     Tokenizer {
    //         buf: line.into().chars().collect(),
    //         pos: 0,
    //         tokens: Vec::new(),
    //     }
    // }
    fn next<'a>(buf: &'a Vec<char>, pos: usize) -> (&'a [char], usize){
        let mut cur = pos;
        let mut head_ch = buf[cur];

        // separator
        if head_ch == '(' || head_ch == ')' || head_ch == ',' {
            return (&buf[pos..pos+1], pos+1)
        }
        // comment
        if head_ch == ';' {
            return (&buf[pos..buf.len()], buf.len());
        } 
        // variables
        if head_ch.is_ascii_alphabetic() || buf[cur] == '.' {
            while cur < buf.len() && (buf[cur].is_ascii_alphabetic() || buf[cur] == ':' || buf[cur] == '.') {
                cur += 1;
            }
            return (&buf[pos..cur], cur);
        }
        // spaces
        if head_ch == ' ' || head_ch == '\t' {
            while cur < buf.len() && (buf[cur] == ' ' || buf[cur] == '\t') {
                cur += 1;
            }
            return (&buf[pos..cur], cur);
        }
        // string
        if head_ch == '"' {
            cur += 1;
            while cur < buf.len() && buf[cur] != '"' {
                cur += 1;
            }
            return (&buf[pos..cur], cur);
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
            //assert!((cur - pos) == 3 || (cur - pos) == 5);
            return (&buf[pos..cur], cur);
        }else if head_ch == '%' {
            cur += 1;
            while cur < buf.len() && buf[cur].is_digit(2) {
                cur += 1;
            }
            //assert!(((cur - pos) == 5 || (cur - pos) == 9));
            return (&buf[pos..cur], cur);
        }else if head_ch.is_digit(10) {
            while cur < buf.len() && buf[cur].is_digit(10) {
                cur += 1;
            }
            return (&buf[pos..cur], cur);
        }
        println!("{:?}", buf);
        panic!();
    }
    pub fn tokenize(line: impl Into<String>) -> Vec<Token>{
        let buf : &Vec<char> = &line.into().chars().collect();
        let mut pos = 0;
        let mut tokens : Vec<Token> = Vec::new();
        println!("{:?}", buf);
        let mut has_op = false;
        while pos < buf.len(){
            let mut cur = pos;
            let mut head_ch = buf[cur];
            // separator
            if head_ch == '(' || head_ch == ')' || head_ch == ',' {
                tokens.push(
                    Token{
                    val: buf[pos..pos+1].to_vec(),
                    kind: match head_ch {
                        ',' | 'x' => TokenKind::Comma,
                        '(' => TokenKind::LParen,
                        ')' => TokenKind::RParen,
                        _ => panic!()
                    }
                });
                pos = pos + 1;
                continue;
            }  
            // comment
            if head_ch == ';' {
                tokens.push(
                    Token{
                    val: buf[pos..buf.len()].to_vec(),
                    kind: TokenKind::Comment,
                });
                break;
            } 
            // variables
            if head_ch.is_ascii_alphabetic() || buf[cur] == '.' {
                let is_head = cur == 0;
                while cur < buf.len() && (buf[cur].is_ascii_alphabetic() || buf[cur] == ':' || buf[cur] == '.') {
                    cur += 1;
                }
                if is_head || buf[cur-1] == ':' || has_op{
                    tokens.push(
                        Token{
                        val: buf[pos..cur].to_vec(),
                        kind: TokenKind::Label,
                    });
                }else if cur - pos == 1 && (buf[cur-1] == 'X' ||buf[cur-1] == 'x' ||buf[cur-1] == 'Y' ||buf[cur-1] == 'y') {
                    tokens.push(
                        Token{
                        val: buf[pos..cur].to_vec(),
                        kind: match buf[cur-1] {
                            'X' | 'x' => TokenKind::X,
                            'Y' | 'y' => TokenKind::Y,
                            _ => panic!(""),
                        }
                    });    
                }else if buf[pos] == '.' {
                    tokens.push(
                        Token{
                        val: buf[pos..cur].to_vec(),
                        kind: TokenKind::Directive,
                    });
                }else{
                    has_op = true;
                    tokens.push(
                        Token{
                        val: buf[pos..cur].to_vec(),
                        kind: TokenKind::Opcode,
                    });
                }
                pos = cur;
                continue;

            }
            // spaces
            if head_ch == ' ' || head_ch == '\t' {
                while cur < buf.len() && (buf[cur] == ' ' || buf[cur] == '\t') {
                    cur += 1;
                }
                tokens.push(
                    Token{
                    val: buf[pos..cur].to_vec(),
                    kind: TokenKind::Spaces,
                });
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
                }else{
                    panic!("\" not found");
                }
                tokens.push(
                    Token{
                    val: buf[pos..cur].to_vec(),
                    kind: TokenKind::String,
                });
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
                    Token{
                        val: buf[pos..cur].to_vec(),
                        kind: if buf[pos] == '#' {
                            TokenKind::Adr
                        }else{
                            TokenKind::Im
                        }
                    }
                );
                pos = cur+1;
                continue;
                //assert!((cur - pos) == 3 || (cur - pos) == 5);
            }else if head_ch == '%' {
                cur += 1;
                while cur < buf.len() && buf[cur].is_digit(2) {
                    cur += 1;
                }
                tokens.push(
                    Token{
                        val: buf[pos..cur].to_vec(),
                        kind: if buf[pos] == '#' {
                            TokenKind::Adr
                        }else{
                            TokenKind::Im
                        }
                    }
                );
                pos = cur+1;
                continue;
                //assert!(((cur - pos) == 5 || (cur - pos) == 9));
            }else if head_ch.is_digit(10) {
                while cur < buf.len() && buf[cur].is_digit(10) {
                    cur += 1;
                }
                tokens.push(
                    Token{
                        val: buf[pos..cur].to_vec(),
                        kind: if buf[pos] == '#' {
                            TokenKind::Adr
                        }else{
                            TokenKind::Im
                        }
                    }
                );
                pos = cur+1;
                continue;
            }
            println!("{:?}", buf);
            panic!();
        }
        tokens
    }
}
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
#[derive(Debug)]
pub enum TokenKind{
    Op,
    Im8,
    Im8Adr,
    Im16,
    X,
    Y,
    A,
    Comma,
    LParen,
    RParen,
    LABEL,
    EOL,
}

#[derive(Debug)]
struct Token{
    val: String,
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
    pub fn new(line: impl Into<String>) -> Tokenizer {
        Tokenizer {
            buf: line.into().chars().collect(),
            pos: 0,
            tokens: Vec::new(),
        }
    }
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
    pub fn tokenize(line: impl Into<String>) {
        let buf : &Vec<char> = &line.into().chars().collect();
        let mut pos = 0;
        while pos < buf.len(){
            let (tokenstr, npos) = Tokenizer::next(buf, pos);
            println!("{:?}", tokenstr);
            pos = npos;
        }

        // println!("tokenize");
        // let ch = self.current();
        // if ch == None { 
        //     println!("None");
        //     self.tokens.push(
        //         Token{
        //             val: "".to_string(),
        //             kind: TokenKind::EOL,
        //         }
        //     );
        //     return;
        // }
        // let ch = ch.unwrap();
        // if ch.is_ascii_alphabetic() || ch == '.' {
        //     // label
        //     self.label(false);
        // }
        // self.space();
        // if self.is_alphabetic() {
        //     self.opecode();
        // }
        // println!("{:?}", self.tokens);
    }
    // fn current(&self) -> Option<char>{
    //     if self.buf.len() <= self.pos{
    //         None
    //     }else{
    //         Some(self.buf[self.pos])
    //     }  
    // }
    // fn opecode(&mut self) {
    //     let head = self.pos;
    //     while self.current().unwrap().is_ascii_alphabetic(){
    //         self.next();
    //     }
    //     self.tokens.push(
    //         Token{
    //             val: self.buf[head..self.pos].iter().collect(),
    //             kind: TokenKind::Op,
    //         }
    //     );
    // }

    // fn pad(&mut self) {
    //     if self.is_space() {
    //         self.space();
    //         self.label(true);
    //     }else{
    //         self.label(false);
    //     }
    // }

    // fn space(&mut self){
    //     while self.pos < self.buf.len() && self.is_space() {
    //         self.pos += 1;
    //     }
    // }
    // fn label(&mut self, colon_required : bool){
    //     if self.buf[self.pos] == '.'{
    //         self.pos += 1;
    //     }
    //     let head = self.pos;
    //     while self.pos < self.buf.len() && self.is_alphabetic() {
    //         self.pos += 1;
    //     }
    //     let ch = self.current();
    //     let mut tail = self.pos;
    //     if colon_required {
    //         if ch.unwrap() != ':' {
    //             panic!("errrr");
    //         }
    //         self.next();
    //         tail += 1;
    //     }else{
    //         if ch != None && ch.unwrap() == ':' {
    //             self.next();
    //             tail += 1;
    //         }
    //     }
        
    //     self.tokens.push(
    //         Token{
    //             val: self.buf[head..tail].iter().collect(),
    //             kind: TokenKind::LABEL,
    //         }
    //     );
    // }
    // fn is_space(&self) -> bool{
    //     let ch = self.buf[self.pos];
    //     ch == ' ' || ch == '\t'
    // }
    // fn is_alphabetic(&self) -> bool{
    //     let ch = self.buf[self.pos];
    //     ch.is_ascii_alphabetic()
    // }
}
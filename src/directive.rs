use std::str::FromStr;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Directive {
    ORG,
    INESPRG,
    INESCHR,
    INESMIR,
    INESMAP,
    BANK,
    DB,
    DW,
    BYTE,
    WORD,
    INCBIN,
}

impl FromStr for Directive {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: &str = &s.to_uppercase();
        match s {
            ".ORG" => Ok(Directive::ORG),
            ".INESPRG" => Ok(Directive::INESPRG),
            ".INESCHR" => Ok(Directive::INESCHR),
            ".INESMIR" => Ok(Directive::INESMIR),
            ".INESMAP" => Ok(Directive::INESMAP),
            ".BANK" => Ok(Directive::BANK),
            ".DB" => Ok(Directive::DB),
            ".DW" => Ok(Directive::DW),
            ".BYTE" => Ok(Directive::BYTE),
            ".WORD" => Ok(Directive::WORD),
            ".INCBIN" => Ok(Directive::INCBIN),
            _ => Err(()),
        }
    }
}

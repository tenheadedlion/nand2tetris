use super::base::*;
use super::error::*;
use super::strutil::Strutil;
use crate::hack_report_less;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>,
    pub cmd_type: Option<CommandType>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            tokens: Vec::new(),
            cmd_type: None,
        }
    }
    pub fn set<'a>(&mut self, expr: &'a str) -> Result<(), Box<HackError>> {
        self.tokens.clear();
        self.cmd_type = None;
        if Lexer::is_empty_line(expr) {
            hack_report_less!("Empty line");
        }
        let expr = expr.trim();
        if expr.starts_with("@") {
            self.cmd_type = Some(CommandType::ACommand);
            self.tokens.push(Token {
                repr: "@".into(),
                token_type: TOKENTYPE::AT,
            });
            Lexer::add_tokens(&mut self.tokens, &expr[1..])
        } else if expr.starts_with("(") && expr.ends_with(")") {
            self.cmd_type = Some(CommandType::LCommand);
            self.tokens.push(Token {
                repr: "(".into(),
                token_type: TOKENTYPE::LEFTBRACE,
            });
            let e = Lexer::add_tokens(&mut self.tokens, &expr[1..expr.len() - 1]);
            self.tokens.push(Token {
                repr: ")".into(),
                token_type: TOKENTYPE::RIGHTBRACE,
            });
            e
        } else {
            self.cmd_type = Some(CommandType::CCommand);
            let mut expr = expr.clone();
            if Strutil::fall_within(expr, "=") {
                let a: Vec<&str> = expr.split('=').collect();
                let dest = Some(a[0]);
                let e = Lexer::add_tokens(&mut self.tokens, dest.unwrap());
                match e {
                    Err(e_) => {
                        return Err(e_);
                    }
                    Ok(_) => {}
                }
                expr = a[1];
                self.tokens.push(Token {
                    repr: "=".into(),
                    token_type: TOKENTYPE::EQUAL,
                });
            }
            if Strutil::fall_within(expr, ";") {
                let a: Vec<&str> = expr.split(';').collect();
                let comp = Some(a[0]);
                let e = Lexer::add_tokens(&mut self.tokens, comp.unwrap());
                match e {
                    Err(e_) => {
                        return Err(e_);
                    }
                    Ok(_) => {}
                }
                expr = a[1];
                self.tokens.push(Token {
                    repr: ";".into(),
                    token_type: TOKENTYPE::SEMICOLON,
                });
            }
            // JUMP
            Lexer::add_tokens(&mut self.tokens, &expr)
        }
    }

    pub fn add_tokens<'a>(v: &mut Vec<Token>, s: &'a str) -> Result<(), Box<HackError>> {
        let subs = s.split_whitespace();
        for sub in subs {
            match Lexer::classify(sub) {
                Ok(t) => v.push(t),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub fn is_empty_line(s: &str) -> bool {
        let iter = s.split_whitespace();
        return iter.count() == 0;
    }

    pub fn classify(s: &str) -> Result<Token, Box<HackError>> {
        lazy_static! {
            static ref NUMBER: Regex = Regex::new(r"^\d+$").unwrap();
            static ref SYMBOL: Regex = Regex::new(r"^[_[:alpha:]]+[_0-9A-Za-z]*$").unwrap();
        }
        if NUMBER.is_match(s) {
            return Ok(Token {
                repr: s.into(),
                token_type: TOKENTYPE::NUMBER,
            });
        } else if SYMBOL.is_match(s) {
            return Ok(Token {
                repr: s.into(),
                token_type: TOKENTYPE::SYMBOL,
            });
        } else {
            return Ok(Token {
                repr: s.into(),
                token_type: TOKENTYPE::EXPRESSION,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new();
        lexer.set("@R2");
        println!("{:?}", lexer.tokens);
        assert_eq!(
            lexer.tokens[0],
            Token {
                repr: "@".into(),
                token_type: TOKENTYPE::AT
            }
        );
        assert_eq!(
            lexer.tokens[1],
            Token {
                repr: "R2".into(),
                token_type: TOKENTYPE::SYMBOL
            }
        );
        lexer.set("@234");
        println!("{:?}", lexer.tokens);
        assert_eq!(
            lexer.tokens[0],
            Token {
                repr: "@".into(),
                token_type: TOKENTYPE::AT
            }
        );
        assert_eq!(
            lexer.tokens[1],
            Token {
                repr: "234".into(),
                token_type: TOKENTYPE::NUMBER
            }
        );
    }
}

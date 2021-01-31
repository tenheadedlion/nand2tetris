use super::base::*;
use super::error::HackError;
use crate::hack_report;
use std::collections::HashMap;
use std::vec::Vec;
/**
 * Recursive Descent Parser
 *
 * COMMAND: ACOMMAND
 *      | CCOMMAND
 *      | LCOMMAND
 * ACOMAND: AT VALUE
 * VALUE: NUMBER | VARIABLE
 * CCOMMAND: DEST COMP JUMP
 * DEST: EMPTY | REGS
 * REGS: M|MD|MD|A|AM|AD||AMD
 *
 */

#[derive(Debug)]
pub struct ParserArg<'a> {
    pub parser: Option<&'a mut Parser>,
    pub tokens: Option<Box<Vec<Token>>>,
    pub index: Option<Box<usize>>,
    pub content: String,
    pub line_num: Option<Box<usize>>,
}

impl<'a> std::fmt::Display for ParserArg<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "[{}]: {} -> tokens: {:?}",
            self.line_num(),
            self.content,
            self.tokens.as_ref().unwrap()
        )
    }
}

#[derive(Debug)]
pub struct LCmdResult {
    pub label: String,
}

#[derive(Debug)]
pub struct CCmdResult {
    pub dest: Option<String>,
    pub comp: Option<String>,
    pub jump: Option<String>,
}

#[derive(Debug)]
pub struct ACmdResult {
    pub value: String,
}

#[derive(Debug)]
pub struct ParserResult {
    pub t: Option<CommandType>,
    pub lr: Option<LCmdResult>,
    pub cr: Option<CCmdResult>,
    pub ar: Option<ACmdResult>,
}
impl ParserResult {
    pub fn clear(&mut self) {
        self.lr = None;
        self.cr = None;
        self.ar = None;
    }
}

impl<'a> ParserArg<'a> {
    pub fn advance(&mut self) {
        let ref mut index = **self.index.as_mut().unwrap();
        *index += 1;
    }
    pub fn line_num(&self) -> usize {
        **self.line_num.as_ref().unwrap()
    }
}

#[derive(Debug)]
pub struct Parser {
    pub map: Option<HashMap<String, usize>>,
    pub varmem: Option<usize>, // variable memory
    pub result: Option<ParserResult>,
}

#[warn(unused_macros)]
macro_rules! create_expect {
    ($func_name:ident, $x:pat) => {
        pub fn $func_name<'a>(
            parg: &'a mut ParserArg<'a>,
        ) -> Result<&'a mut ParserArg<'a>, Box<HackError>> {
            match &parg.tokens.as_ref().unwrap()[**(parg.index.as_ref().unwrap())].token_type {
                $x => {
                    parg.advance();
                    Ok(parg)
                }
                t => hack_report!(parg, format!("Expect {}, but got {:?}", stringify!($x), t)),
            }
        }
    };
}

#[warn(unused_macros)]
macro_rules! create_expect_predefined {
    ($func_name: ident, $x:ident, $y: pat $(, $extra: pat)*) => {
        pub fn $func_name<'a>(
            parg: &'a mut ParserArg<'a>,
        ) -> Result<&'a mut ParserArg<'a>, Box<HackError>> {
            let tokens = parg.tokens.as_ref().unwrap();
            //let parser = parg.parser.as_ref().unwrap();
            let curr = **parg.index.as_ref().unwrap();
            match tokens[curr].token_type {
                $y => {
                    if $x.contains_key(&tokens[curr].repr) {
                        parg.advance();
                        return Ok(parg);
                    }
                    hack_report!(
                        parg,
                        format!(
                            "{} is not defined in table {}!",
                            &tokens[curr].repr,
                            stringify!($x)
                        )
                    )
                }
                $(
                    $extra => {
                        if $x.contains_key(&tokens[curr].repr) {
                            parg.advance();
                            return Ok(parg);
                        }
                        hack_report!(
                            parg,
                            format!(
                                "{} is not defined in table {}!",
                                &tokens[curr].repr,
                                stringify!($x)
                            )
                        )
                    }
                )*
                _ => hack_report!(
                    parg,
                    format!(
                        "Expected {} but found {:?}",
                        stringify!($y),
                        tokens[curr].token_type
                    )
                ),
            }
        }
    };
}

// 一次性解析一组 token
// 返回结果给 HPU
impl Parser {
    pub fn new() -> Parser {
        Parser {
            map: Some(HashMap::new()),
            varmem: Some(16),
            result: Some(ParserResult {
                t: None,
                ar: None,
                cr: None,
                lr: None,
            }),
        }
    }

    pub fn parse_command<'a>(
        parg: &'a mut ParserArg<'a>,
    ) -> Result<&'a mut ParserArg, Box<HackError>> {
        println!("{}", parg);
        let tokens = parg.tokens.as_mut().unwrap();
        match tokens[0].token_type {
            TOKENTYPE::AT => {
                return Parser::expect_a_command(parg);
            }
            TOKENTYPE::LEFTBRACE => Parser::expect_l_command(parg),
            _ => {
                return Parser::expect_c_command(parg);
            }
        }
    }

    create_expect!(expect_leftbrace, TOKENTYPE::LEFTBRACE);
    create_expect!(expect_rightbrace, TOKENTYPE::RIGHTBRACE);
    create_expect!(expect_equal, TOKENTYPE::EQUAL);
    create_expect!(expect_semicolon, TOKENTYPE::SEMICOLON);
    create_expect_predefined!(expect_ccmd_dest, DEST, TOKENTYPE::SYMBOL);
    create_expect_predefined!(
        expect_ccmd_comp,
        COMP,
        TOKENTYPE::EXPRESSION,
        TOKENTYPE::SYMBOL,
        TOKENTYPE::NUMBER
    );
    create_expect_predefined!(expect_ccmd_jump, JUMP, TOKENTYPE::SYMBOL);

    pub fn expect_l_command<'a>(
        parg: &'a mut ParserArg<'a>,
    ) -> Result<&'a mut ParserArg<'a>, Box<HackError>> {
        match Parser::expect_leftbrace(parg)
            .and_then(Parser::expect_symbol_vl)
            .and_then(Parser::expect_rightbrace)
        {
            Ok(arg) => {
                let label = arg.tokens.as_ref().unwrap()[1].repr.clone();
                let parser = arg.parser.as_mut().unwrap();
                let result = parser.result.as_mut().unwrap();
                result.clear();
                result.t = Some(CommandType::LCommand);
                result.lr = Some(LCmdResult {
                    label: label.clone(),
                });
                Ok(arg)
            }
            Err(e) => Err(e),
        }
    }

    pub fn expect_c_command<'a>(
        parg: &'a mut ParserArg<'a>,
    ) -> Result<&'a mut ParserArg<'a>, Box<HackError>> {
        let parser = parg.parser.as_mut().unwrap();
        let result = parser.result.as_mut().unwrap();
        result.clear();
        result.t = Some(CommandType::CCommand);
        result.cr = Some(CCmdResult {
            dest: None,
            comp: None,
            jump: None,
        });
        Parser::expect_c_command_rec(parg)
    }

    pub fn expect_c_command_rec<'a>(
        parg: &'a mut ParserArg<'a>,
    ) -> Result<&'a mut ParserArg<'a>, Box<HackError>> {
        let parser = parg.parser.as_mut().unwrap();
        let result = parser.result.as_mut().unwrap();
        let tokens = parg.tokens.as_ref().unwrap();
        let curr = **parg.index.as_ref().unwrap();
        let len = tokens.len();
        if curr == len {
            return Ok(parg);
        } else if curr + 1 < len && tokens[curr + 1].token_type == TOKENTYPE::EQUAL {
            // this is a defect, but I have no time to fix the expect_* macro
            result.cr.as_mut().unwrap().dest = Some(tokens[curr].repr.clone());
            return Parser::expect_ccmd_dest(parg)
                .and_then(Parser::expect_equal)
                .and_then(Parser::expect_c_command_rec);
        } else if tokens[curr].token_type == TOKENTYPE::SEMICOLON {
            result.cr.as_mut().unwrap().jump = Some(tokens[curr + 1].repr.clone());
            return Parser::expect_semicolon(parg).and_then(Parser::expect_ccmd_jump);
        } else {
            result.cr.as_mut().unwrap().comp = Some(tokens[curr].repr.clone());
            return Parser::expect_ccmd_comp(parg).and_then(Parser::expect_c_command_rec);
        }
    }

    pub fn expect_a_command<'a>(
        parg: &'a mut ParserArg<'a>,
    ) -> Result<&'a mut ParserArg<'a>, Box<HackError>> {
        parg.advance();
        //look ahead
        let tokens = parg.tokens.as_ref().unwrap();
        match tokens[1].token_type {
            TOKENTYPE::NUMBER => Parser::expect_number(parg),
            TOKENTYPE::SYMBOL => Parser::expect_symbol_va(parg),
            _ => hack_report!(parg, "Illegal A command"),
        }
    }

    pub fn expect_number<'a>(
        parg: &'a mut ParserArg<'a>,
    ) -> Result<&'a mut ParserArg<'a>, Box<HackError>> {
        let tokens = parg.tokens.as_ref().unwrap();
        let curr = **parg.index.as_ref().unwrap();
        let parser = parg.parser.as_mut().unwrap();
        match tokens[curr].token_type {
            TOKENTYPE::NUMBER => {
                let result = parser.result.as_mut().unwrap();
                result.t = Some(CommandType::ACommand);
                result.ar = Some(ACmdResult {
                    value: tokens[curr].repr.clone(),
                });
                parg.advance();
                return Ok(parg);
            }
            _ => {
                hack_report!(parg, "No number found")
            }
        }
    }

    pub fn expect_symbol_va<'a>(
        parg: &'a mut ParserArg<'a>,
    ) -> Result<&'a mut ParserArg<'a>, Box<HackError>> {
        let tokens = parg.tokens.as_ref().unwrap();
        let parser = parg.parser.as_mut().unwrap();
        let curr = **parg.index.as_ref().unwrap();
        match tokens[curr].token_type {
            TOKENTYPE::SYMBOL => {
                let result = parser.result.as_mut().unwrap();
                result.clear();
                result.t = Some(CommandType::ACommand);
                result.ar = Some(ACmdResult {
                    value: tokens[curr].repr.clone(),
                });
                parg.advance();
                return Ok(parg);
            }
            _ => hack_report!(parg, "No symbol found"),
        }
    }

    pub fn expect_symbol_vl<'a>(
        parg: &'a mut ParserArg<'a>,
    ) -> Result<&'a mut ParserArg<'a>, Box<HackError>> {
        let tokens = parg.tokens.as_ref().unwrap();
        let curr = **parg.index.as_ref().unwrap();
        println!("{:?}", tokens[curr]);
        match tokens[curr].token_type {
            TOKENTYPE::SYMBOL => {
                if PREDEFINE_SYMBOLS.contains_key(&tokens[curr].repr) {
                    hack_report!(parg, "Using reserved keyword as label is not allowed")
                }
                parg.advance();
                Ok(parg)
            }
            _ => hack_report!(parg, "No label found"),
        }
    }
}

#[cfg(test)]
mod tests {}

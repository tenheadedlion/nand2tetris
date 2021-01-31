use super::base::*;
use super::coder::*;
use super::error::*;
use super::lexer::*;
use super::parser::*;
use super::strutil::Strutil;

pub struct HPU {
    pub path: std::path::PathBuf,
    pub command: Command,
    pub parser: Parser,
    pub lexer: Lexer,
    pub valid_line: usize,
}

impl HPU {
    pub fn new(path: &std::path::PathBuf) -> HPU {
        HPU {
            path: path.clone(),
            command: Command::new(),
            parser: Parser::new(),
            lexer: Lexer::new(),
            valid_line: 0,
        }
    }

    pub fn is_comment(s: &str) -> bool {
        let mut iter = s.chars();
        let _1: char;
        match iter.next() {
            Some(ch) => {
                _1 = ch;
            }
            _ => {
                return false;
            }
        }
        let _2: char;
        match iter.next() {
            Some(ch) => {
                _2 = ch;
            }
            _ => {
                return false;
            }
        }
        if _1 == '/' && _2 == '/' {
            return true;
        }
        return false;
    }

    pub fn should_skip(s: &str) -> bool {
        if Strutil::empty_line(s) {
            return true;
        } else if HPU::is_comment(s) {
            return true;
        } else {
            return false;
        }
    }

    pub fn command_type(s: &String) -> CommandType {
        if s.starts_with("@") {
            return CommandType::ACommand;
        } else if s.starts_with("(") && s.ends_with(")") {
            return CommandType::LCommand;
        } else {
            return CommandType::CCommand;
        }
    }

    pub fn second_pass<'a>(
        &'a mut self,
        num: usize,
        line: &'a str,
    ) -> Result<String, Box<HackError>> {
        if HPU::should_skip(&line) {
            return Ok("".into());
        }
        self.lexer.set(line)?;
        let mut parg = ParserArg {
            parser: Some(&mut self.parser),
            tokens: Some(Box::new(self.lexer.tokens.clone())),
            index: Some(Box::new(0)),
            content: line.into(),
            line_num: Some(Box::new(num)),
        };
        match Parser::parse_command(&mut parg) {
            Ok(arg) => {
                let parser = arg.parser.as_mut().unwrap();
                let result = parser.result.as_ref().unwrap();
                match result.t.as_ref().unwrap() {
                    CommandType::ACommand => Coder::translate_a(
                        parser.map.as_mut().unwrap(),
                        parser.varmem.as_mut().unwrap(),
                        result.ar.as_ref().unwrap(),
                    ),
                    CommandType::CCommand => Coder::translate_c(result.cr.as_ref().unwrap()),
                    CommandType::LCommand => Ok("".to_owned()),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn first_pass<'a>(
        &'a mut self,
        data: &'a (usize, String),
    ) -> Result<(), Box<HackError>> {
        if HPU::should_skip(&data.1) {
            return Ok(());
        }
        let t = HPU::command_type(&data.1);
        if CommandType::LCommand == t {
            self.lexer.set(&data.1)?;
            let mut parg = ParserArg {
                parser: Some(&mut self.parser),
                tokens: Some(Box::new(self.lexer.tokens.clone())),
                index: Some(Box::new(0)),
                content: data.1.clone(),
                line_num: Some(Box::new(data.0)),
            };
            match Parser::parse_command(&mut parg) {
                Ok(arg) => {
                    let parser = arg.parser.as_mut().unwrap();
                    parser.map.as_mut().unwrap().insert(
                        parser
                            .result
                            .as_ref()
                            .unwrap()
                            .lr
                            .as_ref()
                            .unwrap()
                            .label
                            .clone(),
                        self.valid_line,
                    );
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else {
            self.valid_line += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_comand_detection() {
        assert_eq!(Strutil::empty_line("\n"), true);
        assert_eq!(Strutil::empty_line(""), true);
        assert_eq!(HPU::is_comment(""), false);
        assert_eq!(HPU::is_comment("/"), false);
        assert_eq!(HPU::is_comment("//"), true);
        let a = String::from("@INFINITE_LOOP");
        let c = String::from("M=-1");
        let l = String::from("(INFINITE_LOOP)");
        assert_eq!(HPU::command_type(&a), CommandType::ACommand);
        assert_eq!(HPU::command_type(&c), CommandType::CCommand);
        assert_eq!(HPU::command_type(&l), CommandType::LCommand);
    }
    #[test]
    fn test_parse_command() -> Result<(), Box<HackError>> {
        let mut lexer = Lexer::new();
        let mut input = "@R15";
        lexer.set(input)?;
        println!("{:?}", lexer.tokens);
        let mut parser = Parser::new();
        let mut parg = ParserArg {
            parser: Some(&mut parser),
            tokens: Some(Box::new(lexer.tokens.clone())),
            index: Some(Box::new(0)),
            line_num: Some(Box::new(0)),
            content: input.into(),
        };
        match Parser::parse_command(&mut parg) {
            Ok(r) => {
                assert_eq!(**r.index.as_ref().unwrap(), 2);
            }
            Err(e) => panic!("{}", e),
        }

        input = "(    LABEL       )";
        lexer.set(input)?;
        println!("{:?}", lexer.tokens);

        let mut parg = ParserArg {
            parser: Some(&mut parser),
            tokens: Some(Box::new(lexer.tokens.clone())),
            index: Some(Box::new(0)),
            line_num: Some(Box::new(0)),
            content: input.into(),
        };

        match Parser::parse_command(&mut parg) {
            Ok(r) => {
                assert_eq!(**r.index.as_ref().unwrap(), 3);
            }
            Err(e) => panic!("{}", e),
        }

        input = "MD=M-1;JMP";
        lexer.set(input)?;
        println!("{:?}", lexer.tokens);

        let mut parg = ParserArg {
            parser: Some(&mut parser),
            tokens: Some(Box::new(lexer.tokens.clone())),
            index: Some(Box::new(0)),
            line_num: Some(Box::new(0)),
            content: input.into(),
        };

        match Parser::parse_command(&mut parg) {
            Ok(r) => {
                assert_eq!(**r.index.as_ref().unwrap(), 5);
                Ok(())
            }
            Err(e) => panic!("{}", e),
        }
    }
}

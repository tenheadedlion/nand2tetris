use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref DEST: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
        m.insert(String::from("M"), "001");
        m.insert(String::from("D"), "010");
        m.insert(String::from("MD"), "011");
        m.insert(String::from("A"), "100");
        m.insert(String::from("AM"), "101");
        m.insert(String::from("AD"), "110");
        m.insert(String::from("AMD"), "111");
        m
    };
    pub static ref JUMP: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
        m.insert(String::from("JGT"), "001");
        m.insert(String::from("JEQ"), "010");
        m.insert(String::from("JGE"), "011");
        m.insert(String::from("JLT"), "100");
        m.insert(String::from("JNE"), "101");
        m.insert(String::from("JLE"), "110");
        m.insert(String::from("JMP"), "111");
        m
    };
    pub static ref PREDEFINE_SYMBOLS: HashMap<String, i32> = {
        let mut m = HashMap::new();
        m.insert(String::from("SP"), 0);
        m.insert(String::from("LCL"), 1);
        m.insert(String::from("ARG"), 2);
        m.insert(String::from("THIS"), 3);
        m.insert(String::from("THAT"), 4);
        m.insert(String::from("SCREEN"), 16384);
        m.insert(String::from("KBD"), 24576);
        m.insert(String::from("R0"), 0);
        m.insert(String::from("R1"), 1);
        m.insert(String::from("R2"), 2);
        m.insert(String::from("R3"), 3);
        m.insert(String::from("R4"), 4);
        m.insert(String::from("R5"), 5);
        m.insert(String::from("R6"), 6);
        m.insert(String::from("R7"), 7);
        m.insert(String::from("R8"), 8);
        m.insert(String::from("R9"), 9);
        m.insert(String::from("R10"), 10);
        m.insert(String::from("R11"), 11);
        m.insert(String::from("R12"), 12);
        m.insert(String::from("R13"), 13);
        m.insert(String::from("R14"), 14);
        m.insert(String::from("R15"), 15);
        m
    };
    pub static ref COMP: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
        m.insert(String::from("0"),   "0101010");
        m.insert(String::from("1"),   "0111111");
        m.insert(String::from("-1"),  "0111010");
        m.insert(String::from("D"),   "0001100");
        m.insert(String::from("A"),   "0110000");
        m.insert(String::from("M"),   "1110000");
        m.insert(String::from("!D"),  "0001101");
        m.insert(String::from("!A"),  "0110001");
        m.insert(String::from("!M"),  "1110001");
        m.insert(String::from("-D"),  "0001111");
        m.insert(String::from("-A"),  "0110011");
        m.insert(String::from("-M"),  "1110011");
        m.insert(String::from("D+1"), "0011111");
        m.insert(String::from("A+1"), "0110111");
        m.insert(String::from("M+1"), "1110111");
        m.insert(String::from("D-1"), "0001110");
        m.insert(String::from("A-1"), "0110010");
        m.insert(String::from("M-1"), "1110010");
        m.insert(String::from("D+A"), "0000010");
        m.insert(String::from("D+M"), "1000010");
        m.insert(String::from("D-A"), "0010011");
        m.insert(String::from("D-M"), "1010011");
        m.insert(String::from("A-D"), "0000111");
        m.insert(String::from("M-D"), "1000111");
        m.insert(String::from("D&A"), "0000000");
        m.insert(String::from("D&M"), "1000000");
        m.insert(String::from("D|A"), "0010101");
        m.insert(String::from("D|M"), "1010101");
        m
    };
}

#[derive(PartialEq, Debug)]
pub enum CommandType {
    ACommand,
    CCommand,
    LCommand,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TOKENTYPE {
    AT,
    NUMBER,
    SYMBOL,
    LEFTBRACE,
    RIGHTBRACE,
    EQUAL,
    SEMICOLON,
    EXPRESSION,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub repr: String,
    pub token_type: TOKENTYPE,
}

pub struct Command {
    pub lineno: Option<usize>,
    pub lineno_raw: Option<usize>,
    pub command_type: Option<CommandType>,
}

impl Command {
    pub fn new() -> Command {
        Command {
            lineno: None,
            lineno_raw: None,
            command_type: None,
        }
    }
}
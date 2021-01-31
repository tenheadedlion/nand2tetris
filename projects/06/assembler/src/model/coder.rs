use super::base::*;
use super::error::*;
use super::parser::*;
use std::collections::HashMap;

pub struct Coder {}
impl Coder {
    pub fn translate_a<'a>(
        map: &'a mut HashMap<String, usize>,
        varmem: &'a mut usize,
        result: &'a ACmdResult,
    ) -> Result<String, Box<HackError>> {
        match result.value.parse::<i32>() {
            Ok(n) => Ok(format!("0{:015b}", n)),
            Err(_) => match PREDEFINE_SYMBOLS.get(&result.value) {
                Some(n) => Ok(format!("0{:015b}", n)),
                None => match map.get(&result.value) {
                    Some(n) => Ok(format!("0{:015b}", n)),
                    None => {
                        map.insert(result.value.clone(), *varmem);
                        let ret = format!("0{:015b}", *varmem);
                        *varmem += 1;
                        Ok(ret)
                    }
                },
            },
        }
    }

    pub fn translate_c(result: &CCmdResult) -> Result<String, Box<HackError>> {
        let mut ret = String::from("111");
        match &result.comp {
            Some(d) => {
                ret.push_str(COMP.get(d).unwrap());
            }
            None => {
                ret.push_str("000");
            }
        }
        match &result.dest {
            Some(d) => {
                ret.push_str(DEST.get(d).unwrap());
            }
            None => {
                ret.push_str("000");
            }
        }
        match &result.jump {
            Some(d) => {
                ret.push_str(JUMP.get(d).unwrap());
            }
            None => {
                ret.push_str("000");
            }
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    #[test]
    fn test_a_translate() -> Result<(), Box<HackError>> {
        let mut map: HashMap<String, usize> = HashMap::new();
        map.insert("FOO".into(), 20);
        let result = ACmdResult { value: "R0".into() };
        assert_eq!(
            Coder::translate_a(&map, &result).unwrap(),
            "0000000000000000"
        );
        let result = ACmdResult {
            value: "R15".into(),
        };
        assert_eq!(
            Coder::translate_a(&map, &result).unwrap(),
            "0000000000001111"
        );
        let result = ACmdResult {
            value: "FOO".into(),
        };
        assert_eq!(
            Coder::translate_a(&map, &result).unwrap(),
            "0000000000010100"
        );
        Ok(())
    }

    #[test]
    fn test_c_translate() {
        let result = CCmdResult {
            dest: Some("MD".into()),
            comp: Some("M-1".into()),
            jump: Some("JMP".into()),
        };
        assert_eq!(Coder::translate_c(&result).unwrap(), "10111110010111");
    }
}

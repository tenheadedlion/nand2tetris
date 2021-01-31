pub struct Strutil {}
impl Strutil {
    pub fn fall_within(ms: &str, ss: &str) -> bool {
        match ms.find(ss) {
            Some(i) => {
                if i == 0 || i == ms.len() - 1 {
                    return false;
                }
                return true;
            }
            None => false,
        }
    }
    pub fn empty_line(s: &str) -> bool {
        let iter = s.split_whitespace();
        return iter.count() == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test() {
        let s0 = "MD=A-1;JMP";
        assert!(Strutil::fall_within(s0, "="));
        assert!(Strutil::fall_within(s0, ";"));
        let s1 = "=A-1JMP;";
        assert!(!Strutil::fall_within(s1, "="));
        assert!(!Strutil::fall_within(s1, ";"));
    }

    pub fn empty_line(s: &str) -> bool {
        let iter = s.split_whitespace();
        return iter.count() == 0;
    }

    #[test]
    fn test_rust_string() {
        let s = " Hello\tworld\t";
        assert_eq!("Hello\tworld", s.trim());
        assert_eq!(" Hello\tworld\t", s);

        let s2 = " Hello\tworld\t";
        assert_eq!("Hello\tworld", s2.trim());
        assert_eq!(" Hello\tworld\t", s2);

        let s3 = String::from("(LOOP)");
        let label: String = s3.chars().filter(|&c| c != '(' && c != ')').collect();
        assert_eq!(label, "LOOP");

        let s4 = "@R2@";
        let iter: Vec<&str> = s4.split("@").collect();
        let symbol = iter[1];
        assert_eq!(symbol, "R2");
    }
}

#[derive(Debug)]
pub struct HackError {
    pub source_line_num: Option<usize>,
    pub source_line: Option<String>,
    pub comment: String,
}

impl std::fmt::Display for HackError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "[{}]: {}: {}",
            self.source_line_num.as_ref().unwrap(),
            self.source_line.as_ref().unwrap(),
            self.comment
        )
    }
}

impl std::error::Error for HackError {}

#[macro_export]
macro_rules! hack_report {
    ($parg:ident, $comment: expr) => {{
        return Err(Box::new(HackError {
            source_line_num: Some($parg.line_num()),
            source_line:     Some($parg.content.clone()),
            comment:         $comment.to_string(),
        }
    ));
    }};
}

#[macro_export]
macro_rules! hack_report_less {
    ($comment: expr) => {{
        return Err(Box::new(
            HackError {
                source_line_num: None,
                source_line:     None,
                comment:         $comment.to_string(),
            }
        ));
    }};
}

#[derive(Debug)]
pub struct Text {
    pub lines: Vec<Line>,
}

#[derive(Debug)]
pub struct Line {
    pub text: String,
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        let lines = value
            .split("\n")
            .map(|line| Line {
                text: line.to_owned(),
            })
            .collect();
        Self { lines }
    }
}

pub trait Indent {
    fn indent(&self) -> Self;
}

impl Indent for String {
    fn indent(&self) -> Self {
        self.split_inclusive('\n')
            .map(|line| format!("    {line}"))
            .collect::<String>()
            .trim_end_matches(' ')
            .to_string()
    }
}

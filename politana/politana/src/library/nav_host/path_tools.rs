pub trait PathTools {
    fn add_leading_slash(&self) -> String;
    fn split_path(&self) -> Vec<String>;
}

impl PathTools for String {
    fn add_leading_slash(&self) -> String {
        format!("/{}", self)
    }

    fn split_path(&self) -> Vec<String> {
        self.split("/")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }
}

use regex::Regex;

#[derive(Clone)]
pub struct Path {
    pub matcher: Regex,
}

impl Path {
    pub fn new(path: &str) -> Path {
        let mut regex = "^".to_string();
        regex.push_str(path);
        regex.push_str("$");
        Path {
            matcher: Regex::new(&regex).unwrap(),
        }
    }
}
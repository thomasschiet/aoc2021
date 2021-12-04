use std::fs;

pub fn parse_lines_to_ints(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .expect(&*format!("Could not find {}", path))
        .split_whitespace()
        .map(|el| el.parse::<i32>().unwrap())
        .collect()
}

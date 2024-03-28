fn main() {
    println!("Hello, world!");
}

pub struct StrSplit {
}

impl StrSplit {
    pub fn new(haystack: str, delimiter: str) -> Self {
        unimplemented!();
    }
}

pub trait Delimiter {
     fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Iterator for StrSplit {
    type Item = &str;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
    }
}


pub fn until_char(s: &str, c: char) -> str {
    unimplemented!();
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}

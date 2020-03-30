use regex::Regex;


pub trait MatcherTrait {
    fn execute(&self, line: &str) -> bool;
}

pub struct ExtendedRegexpMatcher {
    pattern: Regex,
}
impl ExtendedRegexpMatcher {
    pub fn new(pattern: &str) -> ExtendedRegexpMatcher {
        ExtendedRegexpMatcher {
            pattern: Regex::new(pattern).unwrap(),
        }
    }
}
impl MatcherTrait for ExtendedRegexpMatcher {
    fn execute(&self, line: &str) -> bool {
        self.pattern.is_match(line)
    }
}

pub struct FixedStringsMatcher<'a> {
    pattern: &'a str,
}
impl<'a> FixedStringsMatcher<'a> {
    pub fn new(pattern: &str) -> FixedStringsMatcher {
        FixedStringsMatcher { pattern: pattern }
    }
}
impl<'a> MatcherTrait for FixedStringsMatcher<'a> {
    fn execute(&self, line: &str) -> bool {
        line.contains(self.pattern)
    }
}

pub enum Matcher<'a> {
    ExtendedRegexp(ExtendedRegexpMatcher),
    FixedStrings(FixedStringsMatcher<'a>),
}

pub struct GrepResult {
    pub file_path: String,
    pub hit_lines: Vec<String>,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

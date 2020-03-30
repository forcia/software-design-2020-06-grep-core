use regex::Regex;


pub trait MatcherTrait {
    fn execute(&self, line: &str) -> bool;
}

pub struct ExtendedRegexpMatcher<'a> {
    _original: &'a str,
    pattern: Regex,
}
impl<'a> ExtendedRegexpMatcher<'a> {
    pub fn new(pattern: &'a str) -> ExtendedRegexpMatcher {
        ExtendedRegexpMatcher {
            _original: pattern,
            pattern: Regex::new(pattern).unwrap(),
        }
    }
}
impl<'a> MatcherTrait for ExtendedRegexpMatcher<'a> {
    fn execute(&self, line: &str) -> bool {
        self.pattern.is_match(line)
    }
}

pub struct FixedStringsMatcher<'a> {
    pattern: &'a str,
}
impl<'a> FixedStringsMatcher<'a> {
    pub fn new(pattern: &'a str) -> FixedStringsMatcher {
        FixedStringsMatcher { pattern: pattern }
    }
}
impl<'a> MatcherTrait for FixedStringsMatcher<'a> {
    fn execute(&self, line: &str) -> bool {
        line.contains(self.pattern)
    }
}

pub enum Matcher<'a> {
    ExtendedRegexp(ExtendedRegexpMatcher<'a>),
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

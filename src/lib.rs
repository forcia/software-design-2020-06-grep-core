use regex::Regex;

pub trait MatcherTrait {
    fn execute(&self, line: &str) -> bool;
}

pub struct ExtendedRegexpMatcher {
    pattern: Regex,
}
impl ExtendedRegexpMatcher {
    pub fn new(pattern: String) -> ExtendedRegexpMatcher {
        ExtendedRegexpMatcher {
            pattern: Regex::new(&pattern).unwrap(),
        }
    }
}
impl MatcherTrait for ExtendedRegexpMatcher {
    fn execute(&self, line: &str) -> bool {
        self.pattern.is_match(line)
    }
}

pub struct FixedStringsMatcher {
    pattern: String,
}
impl FixedStringsMatcher {
    pub fn new(pattern: String) -> FixedStringsMatcher {
        FixedStringsMatcher { pattern: pattern }
    }
}
impl MatcherTrait for FixedStringsMatcher {
    fn execute(&self, line: &str) -> bool {
        line.contains(&self.pattern)
    }
}

pub enum Matcher {
    ExtendedRegexp(ExtendedRegexpMatcher),
    FixedStrings(FixedStringsMatcher),
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

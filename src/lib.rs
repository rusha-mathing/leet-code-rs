pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::is_match_chars(s.chars(), p.chars())
    }

    pub fn is_match_chars<T>(mut s: T, mut p: T) -> bool
    where
        T: Iterator<Item = char> + Clone,
    {
        let p_ch = p.next();
        return if p_ch == None {
            s.next() == None
        } else if p.clone().next().map_or(false, |ch| ch == '*') {
            let p_ch = p_ch.unwrap();
            let mut s_clone = s.clone();
            let mut s_ch = s_clone.next();
            assert_eq!(
                p.next(),
                Some('*'),
                "Excepted that this if branch is matching the star case"
            );
            while s_ch.map_or(false, |ch| is_match(ch, p_ch)) {
                if Self::is_match_chars(s_clone.clone(), p.clone()) {
                    return true;
                }
                s_ch = s_clone.next();
            }
            return Self::is_match_chars(s, p);
        } else if s.clone().next() == None {
            false
        } else if is_match(s.next().unwrap(), p_ch.unwrap()) {
            Self::is_match_chars(s, p)
        } else {
            false
        };
    }
}

fn is_match(s: char, p: char) -> bool {
    s == p || p == '.'
}

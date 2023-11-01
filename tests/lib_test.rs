
#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    fn helper(s: &str, expect: &[&str]) {
        assert!(expect.contains(&Solution::longest_palindrome(s.to_owned()).as_str()));
    }

    #[test]
    fn test_1() {
        helper("babad", &["bab", "aba"]);
    }
    
    #[test]
    fn test_2() {
        helper("cbbd", &["bb"]);
    }
    
    #[test]
    fn test_3() {
        helper("a", &["a"]);
    }
    
    #[test]
    fn test_4() {
        helper("ac", &["a", "c"]);
    }
    
    #[test]
    fn test_5() {
        helper("bb", &["bb"]);
    }
    
    #[test]
    fn test_6() {
        helper("abcbe", &["bcb"]);
    }
    
    #[test]
    fn test_7() {
        helper("abcde", &["a", "b", "c", "d", "e"]);
    }
    
    #[test]
    fn test_8() {
        helper("babadbbabb", &["bbabb"]);
    }
    
    #[test]
    fn test_9() {
        helper("aaaa", &["aaaa"]);
    }
    
    #[test]
    fn test_10() {
        helper("aaabaaaa", &["aaabaaa"]);
    }
    
    #[test]
    fn test_11() {
        helper("racecar", &["racecar"]);
    }
    
    #[test]
    fn test_12() {
        helper("level", &["level"]);
    }
    
    #[test]
    fn test_13() {
        helper("madam", &["madam"]);
    }
    
    #[test]
    fn test_14() {
        helper("deified", &["deified"]);
    }
    
    #[test]
    fn test_15() {
        helper("helloolleh", &["helloolleh"]);
    }
    
    #[test]
    fn test_16() {
        helper("aaaaab", &["aaaaa"]);
    }
    
    #[test]
    fn test_17() {
        helper("abcdeedcba", &["abcdeedcba"]);
    }
    
    #[test]
    fn test_18() {
        helper("abcdedcba", &["abcdedcba"]);
    }
    
    #[test]
    fn test_19() {
       helper ("abccba", &["abccba"]);
    }
    
    #[test]
    fn test_20() {
       helper ("abcabcabc", &["a", "b", "c"]);
    }
}


#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    fn helper(vec: Vec<&str>, except: &str) {
        assert_eq!(
            Solution::longest_common_prefix(vec.into_iter().map(ToOwned::to_owned).collect()),
            except
        );
    }

    #[test]
    fn test_1() {
        helper(vec!["flower", "flow", "flight"], "fl");
    }
    
    #[test]
    fn test_2() {
        helper(vec!["dog", "racecar", "car"], "");
    }
    
    #[test]
    fn test_3() {
        helper(vec!["hello", "hell", "heaven"], "he");
    }
    
    #[test]
    fn test_4() {
        helper(vec!["test", "test", "test"], "test");
    }
    
    #[test]
    fn test_5() {
        helper(vec!["testing", "test", "tested"], "test");
    }
    
    #[test]
    fn test_6() {
        helper(vec!["a", "b", "c"], "");
    }
    
    #[test]
    fn test_7() {
        helper(vec!["abc", "abcd", "abcde"], "abc");
    }
    
    #[test]
    fn test_8() {
        helper(vec!["abcd", "abccd", "abcccd"], "abc");
    }
    
    #[test]
    fn test_9() {
        helper(vec!["go", "google", "goose"], "go");
    }
    
    #[test]
    fn test_10() {
        helper(vec!["home", "homework", "holy"], "ho");
    }
    
    #[test]
    fn test_11() {
        helper(vec!["in", "input", "information"], "in");
    }
    
    #[test]
    fn test_12() {
        helper(vec!["long", "longer", "longest"], "long");
    }
    
    #[test]
    fn test_13() {
        helper(vec!["plane", "planet", "plank"], "plan");
    }
    
    #[test]
    fn test_14() {
        helper(vec!["super", "superb", "superman"], "super");
    }
    
    #[test]
    fn test_15() {
        helper(vec!["quick", "qu", "quiet"], "qu");
    }
    
    #[test]
    fn test_16() {
        helper(vec!["race", "racism", "rack"], "rac");
    }
    
    #[test]
    fn test_17() {
        helper(vec!["test", "tested", "te"], "te");
    }
    
    #[test]
    fn test_18() {
        helper(vec!["win", "wind", "window"], "win");
    }
    
    #[test]
    fn test_19() {
        helper(vec!["zoom", "zoo", "zoologist"], "zoo");
    }
    
    #[test]
    fn test_20() {
        helper(vec!["ab", "ac", "ad"], "a");
    }
    
    #[test]
    fn test_21() {
        helper(vec!["mn", "mo", "mp"], "m");
    }
    
    #[test]
    fn test_22() {
        helper(vec!["xy", "x", "xyz"], "x");
    }
    
    #[test]
    fn test_23() {
        helper(vec!["zz", "z", "zzz"], "z");
    }
    
    #[test]
    fn test_24() {
        helper(vec!["cc", "cc", "cc"], "cc");
    }
    
    #[test]
    fn test_25() {
        helper(vec!["", "", ""], "");
    }
    
}
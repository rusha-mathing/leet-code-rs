#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    fn helper(s: &str, output: i32) {
        assert_eq!(Solution::length_of_longest_substring(s.into()), output);
    }

    #[test]
    fn test_1() {
        helper("abcabcbb", 3);
    }

    #[test]
    fn test_2() {
        helper("bbbbb", 1);
    }

    #[test]
    fn test_3() {
        helper("pwwkew", 3);
    }

    #[test]
    fn test_4() {
        helper("", 0);
    }

    #[test]
    fn test_5() {
        helper("aab", 2);
    }

    #[test]
    fn test_6() {
        helper("dvdf", 3);
    }

    #[test]
    fn test_7() {
        helper("abcdefg", 7);
    }

    #[test]
    fn test_8() {
        helper("aaabcd", 4);
    }

    #[test]
    fn test_9() {
        helper("ababcccccc", 3);
    }

    #[test]
    fn test_10() {
        helper("bbabb", 2);
    }

    #[test]
    fn test_11() {
        helper("abcde", 5);
    }

    #[test]
    fn test_12() {
        helper("aabbccddeeff", 2);
    }

    #[test]
    fn test_13() {
        helper("abcdefabcdef", 6);
    }

    #[test]
    fn test_14() {
        helper("a", 1);
    }

    #[test]
    fn test_15() {
        helper("xyzxyz", 3);
    }

    #[test]
    fn test_16() {
        helper("yxzzy", 4);
    }

    #[test]
    fn test_17() {
        helper("abcxyzxyzab", 6);
    }

    #[test]
    fn test_18() {
        helper("abababababab", 2);
    }

    #[test]
    fn test_19() {
        helper("qwertyqwertyqwerty", 6);
    }

    #[test]
    fn test_20() {
        helper("abababa", 2);
    }

    #[test]
    fn test_21() {
        helper("0010010", 3);
    }

    #[test]
    fn test_22() {
        helper("zqkmndz", 6);
    }

    #[test]
    fn test_23() {
        helper("x", 1);
    }

    #[test]
    fn test_24() {
        helper("uuu", 1);
    }

    #[test]
    fn test_25() {
        helper("racecar", 4);
    }

    #[test]
    fn test_26() {
        helper("bbabababa", 2);
    }

    #[test]
    fn test_27() {
        helper("zxcvbnmasdfghjklqwerty", 21);
    }

    #[test]
    fn test_28() {
        helper("abacadae", 5);
    }

    #[test]
    fn test_29() {
        helper("abcbdefghijklmnop", 14);
    }

    #[test]
    fn test_30() {
        helper("abcABC123", 9);
    }

    #[test]
    fn test_31() {
        helper("gggggh", 2);
    }

    #[test]
    fn test_32() {
        helper("hello world", 6);
    }

    #[test]
    fn test_33() {
        helper("xyyzuvwxy", 6);
    }

    #[test]
    fn test_34() {
        helper("ddddddeeeeeeffff", 3);
    }

    #[test]
    fn test_35() {
        helper("aabbbcc", 2);
    }

    #[test]
    fn test_36() {
        helper("bbbbbbbbbb", 1);
    }

    #[test]
    fn test_37() {
        helper("abcdefghijk", 11);
    }

    #[test]
    fn test_38() {
        helper("aabbccddee", 2);
    }

    #[test]
    fn test_39() {
        helper("mnopqrstuvw", 12);
    }

    #[test]
    fn test_40() {
        helper("abbbbaaa", 2);
    }
}

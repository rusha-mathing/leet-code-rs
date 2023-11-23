#[cfg(test)]
mod tests {
    use leet_code_rs::*;

    fn helper(n: i32, except: &str) {
        assert_eq!(Solution::int_to_roman(n), except)
    }

    #[test]
    fn test_1() {
        helper(3, "III");
    }

    #[test]
    fn test_2() {
        helper(4, "IV");
    }

    #[test]
    fn test_3() {
        helper(9, "IX");
    }

    #[test]
    fn test_4() {
        helper(58, "LVIII");
    }

    #[test]
    fn test_5() {
        helper(1994, "MCMXCIV");
    }

    #[test]
    fn test_6() {
        helper(3999, "MMMCMXCIX");
    }

    #[test]
    fn test_7() {
        helper(8, "VIII");
    }

    #[test]
    fn test_8() {
        helper(99, "XCIX");
    }

    #[test]
    fn test_9() {
        helper(92, "XCII");
    }

    #[test]
    fn test_10() {
        helper(46, "XLVI");
    }

    #[test]
    fn test_11() {
        helper(16, "XVI");
    }

    #[test]
    fn test_12() {
        helper(29, "XXIX");
    }

    #[test]
    fn test_13() {
        helper(21, "XXI");
    }

    #[test]
    fn test_14() {
        helper(293, "CCXCIII");
    }

    #[test]
    fn test_15() {
        helper(846, "DCCCXLVI");
    }

    #[test]
    fn test_16() {
        helper(769, "DCCLXIX");
    }

    #[test]
    fn test_17() {
        helper(555, "DLV");
    }

    #[test]
    fn test_18() {
        helper(1111, "MCXI");
    }

    #[test]
    fn test_19() {
        helper(888, "DCCCLXXXVIII");
    }

    #[test]
    fn test_20() {
        helper(1234, "MCCXXXIV");
    }

    #[test]
    fn test_21() {
        helper(3000, "MMM");
    }

    #[test]
    fn test_22() {
        helper(123, "CXXIII");
    }

    #[test]
    fn test_23() {
        helper(2876, "MMDCCCLXXVI");
    }

    #[test]
    fn test_24() {
        helper(1764, "MDCCLXIV");
    }

    #[test]
    fn test_25() {
        helper(1989, "MCMLXXXIX");
    }

    #[test]
    fn test_26() {
        helper(62, "LXII");
    }

    #[test]
    fn test_27() {
        helper(141, "CXLI");
    }

    #[test]
    fn test_28() {
        helper(777, "DCCLXXVII");
    }

    #[test]
    fn test_29() {
        helper(545, "DXLV");
    }

    #[test]
    fn test_30() {
        helper(666, "DCLXVI");
    }

    #[test]
    fn test_31() {
        helper(3995, "MMMCMXCV");
    }

    #[test]
    fn test_32() {
        helper(2001, "MMI");
    }

    #[test]
    fn test_33() {
        helper(1000, "M");
    }

    #[test]
    fn test_34() {
        helper(500, "D");
    }

    #[test]
    fn test_35() {
        helper(100, "C");
    }

    #[test]
    fn test_36() {
        helper(50, "L");
    }

    #[test]
    fn test_37() {
        helper(10, "X");
    }

    #[test]
    fn test_38() {
        helper(5, "V");
    }

    #[test]
    fn test_39() {
        helper(1997, "MCMXCVII");
    }

    #[test]
    fn test_40() {
        helper(1916, "MCMXVI");
    }

    #[test]
    fn test_41() {
        helper(1008, "MVIII");
    }

    #[test]
    fn test_42() {
        helper(1500, "MD");
    }

    #[test]
    fn test_43() {
        helper(2500, "MMD");
    }

    #[test]
    fn test_44() {
        helper(3500, "MMMD");
    }

    #[test]
    fn test_45() {
        helper(600, "DC");
    }

    #[test]
    fn test_46() {
        helper(700, "DCC");
    }

    #[test]
    fn test_47() {
        helper(200, "CC");
    }

    #[test]
    fn test_48() {
        helper(300, "CCC");
    }

    #[test]
    fn test_49() {
        helper(400, "CD");
    }

    #[test]
    fn test_50() {
        helper(1100, "MC");
    }

    #[test]
    fn test_51() {
        helper(900, "CM");
    }

    #[test]
    fn test_52() {
        helper(110, "CX");
    }

    #[test]
    fn test_53() {
        helper(120, "CXX");
    }

    #[test]
    fn test_54() {
        helper(130, "CXXX");
    }

    #[test]
    fn test_55() {
        helper(140, "CXL");
    }

    #[test]
    fn test_56() {
        helper(150, "CL");
    }

    #[test]
    fn test_57() {
        helper(160, "CLX");
    }

    #[test]
    fn test_58() {
        helper(170, "CLXX");
    }

    #[test]
    fn test_59() {
        helper(180, "CLXXX");
    }

    #[test]
    fn test_60() {
        helper(190, "CXC");
    }

    #[test]
    fn test_61() {
        helper(201, "CCI");
    }

    #[test]
    fn test_62() {
        helper(202, "CCII");
    }

    #[test]
    fn test_63() {
        helper(203, "CCIII");
    }

    #[test]
    fn test_64() {
        helper(204, "CCIV");
    }

    #[test]
    fn test_65() {
        helper(205, "CCV");
    }

    #[test]
    fn test_66() {
        helper(206, "CCVI");
    }

    #[test]
    fn test_67() {
        helper(207, "CCVII");
    }

    #[test]
    fn test_68() {
        helper(208, "CCVIII");
    }

    #[test]
    fn test_69() {
        helper(209, "CCIX");
    }

    #[test]
    fn test_70() {
        helper(222, "CCXXII");
    }

    #[test]
    fn test_71() {
        helper(333, "CCCXXXIII");
    }

    #[test]
    fn test_72() {
        helper(444, "CDXLIV");
    }

    #[test]
    fn test_73() {
        helper(555, "DLV");
    }

    #[test]
    fn test_74() {
        helper(666, "DCLXVI");
    }

    #[test]
    fn test_75() {
        helper(777, "DCCLXXVII");
    }

    #[test]
    fn test_76() {
        helper(888, "DCCCLXXXVIII");
    }

    #[test]
    fn test_77() {
        helper(999, "CMXCIX");
    }

    #[test]
    fn test_78() {
        helper(2000, "MM");
    }

    #[test]
    fn test_79() {
        helper(3000, "MMM");
    }

    #[test]
    fn test_80() {
        helper(3999, "MMMCMXCIX");
    }
}

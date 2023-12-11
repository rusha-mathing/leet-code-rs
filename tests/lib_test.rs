#[cfg(test)]
mod tests {
    use leet_code_rs::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::letter_combinations(String::from("23")),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn test_2() {
        assert!(Solution::letter_combinations(String::from("")).is_empty());
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::letter_combinations(String::from("2")),
            vec!["a", "b", "c"]
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::letter_combinations(String::from("7")),
            vec!["p", "q", "r", "s"]
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::letter_combinations(String::from("234")),
            vec![
                "adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg", "bdh", "bdi",
                "beg", "beh", "bei", "bfg", "bfh", "bfi", "cdg", "cdh", "cdi", "ceg", "ceh", "cei",
                "cfg", "cfh", "cfi"
            ]
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::letter_combinations(String::from("89")),
            vec!["tw", "tx", "ty", "tz", "uw", "ux", "uy", "uz", "vw", "vx", "vy", "vz"]
        );
    }

    #[test]
    fn test_7() {
        assert_eq!(
            Solution::letter_combinations(String::from("45")),
            vec!["gj", "gk", "gl", "hj", "hk", "hl", "ij", "ik", "il"]
        );
    }

    #[test]
    fn test_8() {
        assert_eq!(
            Solution::letter_combinations(String::from("67")),
            vec!["pm", "pn", "po", "qm", "qn", "qo", "rm", "rn", "ro", "sm", "sn", "so"]
        );
    }

    #[test]
    fn test_9() {
        assert_eq!(
            Solution::letter_combinations(String::from("22")),
            vec!["aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc"]
        );
    }

    #[test]
    fn test_10() {
        assert_eq!(
            Solution::letter_combinations(String::from("888")),
            vec![
                "ttt", "ttu", "ttv", "tut", "tuu", "tuv", "tvt", "tvu", "tvv", "utt", "utu", "utv",
                "uut", "uuu", "uuv", "uvt", "uvu", "uvv", "vtt", "vtu", "vtv", "vut", "vuu", "vuv",
                "vvt", "vvu", "vvv"
            ]
        );
    }

    #[test]
    fn test_11() {
        assert_eq!(
            Solution::letter_combinations(String::from("3")),
            vec!["d", "e", "f"]
        );
    }

    #[test]
    fn test_12() {
        assert_eq!(
            Solution::letter_combinations(String::from("5")),
            vec!["j", "k", "l"]
        );
    }

    #[test]
    fn test_13() {
        assert_eq!(
            Solution::letter_combinations(String::from("9")),
            vec!["w", "x", "y", "z"]
        );
    }

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::letter_combinations(String::from("777")),
            vec![
                "ppp", "ppq", "ppr", "pps", "pqp", "pqq", "pqr", "pqs", "prp", "prq", "prr", "prs",
                "psp", "psq", "psr", "pss", "qpp", "qpq", "qpr", "qps", "qqp", "qqq", "qqr", "qqs",
                "qrp", "qrq", "qrr", "qrs", "qsp", "qsq", "qsr", "qss", "rpp", "rpq", "rpr", "rps",
                "rqp", "rqq", "rqr", "rqs", "rrp", "rrq", "rrr", "rrs", "rsp", "rsq", "rsr", "rss",
                "spp", "spq", "spr", "sps", "sqp", "sqq", "sqr", "sqs", "srp", "srq", "srr", "srs",
                "ssp", "ssq", "ssr", "sss"
            ]
        );
    }

    #[test]
    fn test_15() {
        assert_eq!(
            Solution::letter_combinations(String::from("422")),
            vec![
                "gaa", "gab", "gac", "gba", "gbb", "gbc", "gca", "gcb", "gcc", "haa", "hab", "hac",
                "hba", "hbb", "hbc", "hca", "hcb", "hcc", "iaa", "iab", "iac", "iba", "ibb", "ibc",
                "ica", "icb", "icc"
            ]
        );
    }

    #[test]
    fn test_16() {
        assert_eq!(
            Solution::letter_combinations(String::from("99")),
            vec![
                "ww", "wx", "wy", "wz", "xw", "xx", "xy", "xz", "yw", "yx", "yy", "yz", "zw", "zx",
                "zy", "zz"
            ]
        );
    }

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations(String::from("69")),
            vec!["mw", "mx", "my", "mz", "nw", "nx", "ny", "nz", "ow", "ox", "oy", "oz"]
        );
    }

    #[test]
    fn test_18() {
        assert_eq!(
            Solution::letter_combinations(String::from("324")),
            vec![
                "dag", "dah", "dai", "dbg", "dbh", "dbi", "dcg", "dch", "dci", "eag", "eah", "eai",
                "ebg", "ebh", "ebi", "ecg", "ech", "eci", "fag", "fah", "fai", "fbg", "fbh", "fbi",
                "fcg", "fch", "fci"
            ]
        );
    }

    #[test]
    fn test_19() {
        assert_eq!(
            Solution::letter_combinations(String::from("937")),
            vec![
                "wwp", "wwq", "wwr", "wws", "wxp", "wxq", "wxr", "wxs", "wyp", "wyq", "wyr", "wys",
                "wzp", "wzq", "wzr", "wzs", "xwp", "xwq", "xwr", "xws", "xxp", "xxq", "xxr", "xxs",
                "xyp", "xyq", "xyr", "xys", "xzp", "xzq", "xzr", "xzs", "ywp", "ywq", "ywr", "yws",
                "yxp", "yxq", "yxr", "yxs", "yyp", "yyq", "yyr", "yys", "yzp", "yzq", "yzr", "yzs",
                "zwp", "zwq", "zwr", "zws", "zxp", "zxq", "zxr", "zxs", "zyp", "zyq", "zyr", "zys",
                "zzp", "zzq", "zzr", "zzs"
            ]
        );
    }

    #[test]
    fn test_20() {
        assert_eq!(
            Solution::letter_combinations(String::from("674")),
            vec![
                "mpg", "mph", "mpi", "mqg", "mqh", "mqi", "mrg", "mrh", "mri", "msg", "msh", "msi",
                "npg", "nph", "npi", "nqg", "nqh", "nqi", "nrg", "nrh", "nri", "nsg", "nsh", "nsi",
                "opg", "oph", "opi", "oqg", "oqh", "oqi", "org", "orh", "ori", "osg", "osh", "osi"
            ]
        );
    }

    #[test]
    fn test_21() {
        assert_eq!(
            Solution::letter_combinations(String::from("228")),
            vec![
                "aat", "aau", "aav", "abt", "abu", "abv", "act", "acu", "acv", "bat", "bau", "bav",
                "bbt", "bbu", "bbv", "bct", "bcu", "bcv", "cat", "cau", "cav", "cbt", "cbu", "cbv",
                "cct", "ccu", "ccv"
            ]
        );
    }

    #[test]
    fn test_22() {
        assert_eq!(
            Solution::letter_combinations(String::from("733")),
            vec![
                "pde", "pdf", "pdg", "peq", "per", "pes", "pfe", "pff", "pfg", "pqe", "pqf", "pqg",
                "pre", "prf", "prg", "pse", "psf", "psg", "qde", "qdf", "qdg", "qeq", "qer", "qes",
                "qfe", "qff", "qfg", "qqe", "qqf", "qqg", "qre", "qrf", "qrg", "qse", "qsf", "qsg",
                "rde", "rdf", "rdg", "req", "rer", "res", "rfe", "rff", "rfg", "rqe", "rqf", "rqg",
                "rre", "rrf", "rrg", "rse", "rsf", "rsg", "sde", "sdf", "sdg", "seq", "ser", "ses",
                "sfe", "sff", "sfg", "sqe", "sqf", "sqg", "sre", "srf", "srg", "sse", "ssf", "ssg"
            ]
        );
    }

    #[test]
    fn test_23() {
        assert_eq!(
            Solution::letter_combinations(String::from("997")),
            vec![
                "www", "wwx", "wwy", "wwz", "wxw", "wxx", "wxy", "wxz", "wyw", "wyx", "wyy", "wyz",
                "wzw", "wzx", "wzy", "wzz", "xww", "xwx", "xwy", "xwz", "xxw", "xxx", "xxy", "xxz",
                "xyw", "xyx", "xyy", "xyz", "xzw", "xzx", "xzy", "xzz", "yww", "ywx", "ywy", "ywz",
                "yxw", "yxx", "yxy", "yxz", "yyw", "yyx", "yyy", "yyz", "yzw", "yzx", "yzy", "yzz",
                "zww", "zwx", "zwy", "zwz", "zxw", "zxx", "zxy", "zxz", "zyw", "zyx", "zyy", "zyz",
                "zzw", "zzx", "zzy", "zzz"
            ]
        );
    }

    #[test]
    fn test_24() {
        assert_eq!(
            Solution::letter_combinations(String::from("144")),
            vec![
                "agd", "age", "agf", "ahd", "ahe", "ahf", "aid", "aie", "aif", "bgd", "bge", "bgf",
                "bhd", "bhe", "bhf", "bid", "bie", "bif", "cgd", "cge", "cgf", "chd", "che", "chf",
                "cid", "cie", "cif"
            ]
        );
    }

    #[test]
    fn test_25() {
        assert_eq!(
            Solution::letter_combinations(String::from("856")),
            vec![
                "tjm", "tjn", "tjo", "tkm", "tkn", "tko", "tlm", "tln", "tlo", "ujm", "ujn", "ujo",
                "ukm", "ukn", "uko", "ulm", "uln", "ulo", "vjm", "vjn", "vjo", "vkm", "vkn", "vko",
                "vlm", "vln", "vlo"
            ]
        );
    }
}

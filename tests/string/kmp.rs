use algorithms_rs::string::Kmp;

#[test]
fn test_kmp() {
    let s = "AABAACAADAABAABA";
    let s1 = format!("{s}A");
    let pat = vec!["AABA", "B", "E", s, s1.as_str()];
    let expected = vec![vec![0, 9, 12], vec![2, 11, 14], vec![], vec![0], vec![]];

    for (p, e) in pat.iter().zip(expected) {
        assert_eq!(s.kmp(p), e);
    }
}

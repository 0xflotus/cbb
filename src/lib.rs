pub mod util;

#[test]
fn test() {
    assert!(util::cbb::int_to_bal_ternary(1) == "000+");
    assert!(util::cbb::int_to_bal_ternary(2) == "00+-");
    assert!(util::cbb::int_to_bal_ternary(3) == "00+0");
    assert!(util::cbb::int_to_bal_ternary(17) == "+-0-");
    assert!(util::cbb::int_to_bal_ternary(20) == "+-+-");
    assert!(util::cbb::int_to_bal_ternary(200) == "+-+++-");
}

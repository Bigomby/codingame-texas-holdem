extern crate poker;

#[test]
fn test_high_card() {
    let player1 = String::from("8D 7C");
    let player2 = String::from("7D 6C");
    let table = String::from("KS 9D 5C 3S 2D");

    let result = poker::play(player1, player2, table);
    assert_eq!(result, "1 HIGH_CARD K9875");
}

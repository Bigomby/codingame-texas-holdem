// MIT License
//
// Copyright (c) 2016 Diego Fernández Barrera
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::string::String;

mod consts; // Constants
mod types;  // Types definition
mod card;   // Card struct with its tests
mod hand;   // Hand struct with its tests

/**
 * play Initializes a game with 2 players with 2 cards each one and 5 community cards. It will
 * try to find the highest hand for every player. Returns number of the player with the highest
 * hand, the name of the hand and the cards.
 *
 * Example:
 *  INPUT                       OUTPUT
 *      8D 7C                       1 HIGH_CARD K9875
 *      7D 6C
 *      KS 9D 5C 3S 2D
 */
pub fn play(player1: String, player2: String, table: String) -> String {
    // Separate cards
    let player1_str_cards: Vec<&str> = player1.split_whitespace().collect();
    let player2_str_cards: Vec<&str> = player2.split_whitespace().collect();
    let table_str_cards: Vec<&str> = table.split_whitespace().collect();

    // Create instances of cards cards
    let player1_card1 = card::Card::new(player1_str_cards[0].chars().nth(0).unwrap(),
                                        player1_str_cards[0].chars().nth(1).unwrap());
    let player1_card2 = card::Card::new(player1_str_cards[1].chars().nth(0).unwrap(),
                                        player1_str_cards[1].chars().nth(1).unwrap());

    let player2_card1 = card::Card::new(player2_str_cards[0].chars().nth(0).unwrap(),
                                        player2_str_cards[0].chars().nth(1).unwrap());
    let player2_card2 = card::Card::new(player2_str_cards[1].chars().nth(0).unwrap(),
                                        player2_str_cards[1].chars().nth(1).unwrap());

    let table_card1 = card::Card::new(table_str_cards[0].chars().nth(0).unwrap(),
                                      table_str_cards[0].chars().nth(1).unwrap());
    let table_card2 = card::Card::new(table_str_cards[1].chars().nth(0).unwrap(),
                                      table_str_cards[1].chars().nth(1).unwrap());
    let table_card3 = card::Card::new(table_str_cards[2].chars().nth(0).unwrap(),
                                      table_str_cards[2].chars().nth(1).unwrap());
    let table_card4 = card::Card::new(table_str_cards[3].chars().nth(0).unwrap(),
                                      table_str_cards[3].chars().nth(1).unwrap());
    let table_card5 = card::Card::new(table_str_cards[4].chars().nth(0).unwrap(),
                                      table_str_cards[4].chars().nth(1).unwrap());

    // Create two collections of cards for the two players and another one for the table
    let player1: types::Hole = [&player1_card1, &player1_card2];
    let player2: types::Hole = [&player2_card1, &player2_card2];
    let table: types::Table =
        [&table_card1, &table_card2, &table_card3, &table_card4, &table_card5];

    // Compute the best hand for every player
    let hand1 = hand::Hand::new(player1, table);
    let hand2 = hand::Hand::new(player2, table);

    // Check which hand is better
    let mut result = String::new();
    if hand1 > hand2 {
        result.push_str("1 ");
        result.push_str(hand1.get_hand_type());
        result.push_str(" ");
        result.push_str(&hand1.to_string());
    } else if hand1 > hand2 {
        result.push_str("2 ");
        result.push_str(hand2.get_hand_type());
        result.push_str(" ");
        result.push_str(&hand2.to_string());
    } else {
        result.push_str("DRAW");
    }

    result
}

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

// External imports
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;

// Internal imports
use types;
use consts;
use card::CardValue;
use card::CardSuit;
use card::Card;

/**
 * HandType can be any of the possible defined hands. Ordered by value.
 */
#[derive(PartialEq)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

/**
 * Hand represents a combination of 5 cards with together build a HandType.
 */
pub struct Hand<'a> {
    cards: [&'a Card; consts::HAND_SIZE], // Combination of the cards that builds the hand
    hand_type: HandType, // Type of the hand
}

/**
 * Implement Eq and PartialEq for the Hand so it's possible to check if the have the same value
 */
impl<'a> Eq for Hand<'a> {}
impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Hand) -> bool {
        for i in 0..consts::HAND_SIZE {
            if self.cards[i] != other.cards[i] {
                return false;
            }
        }
        true
    }
}

/**
 * Implement Ord and PartialOrd for the Hand so it's possible to check which hand has more value
 */
impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Hand) -> Ordering {
        for i in 0..consts::HAND_SIZE {
            let ordering: Ordering = self.cards[i].value.cmp(&other.cards[i].value);
            if ordering == Ordering::Equal {
                continue;
            } else {
                return ordering;
            }
        }
        Ordering::Equal
    }
}
impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/**
 * Implement Display for Hand so it's possible to obtain the alias of the card values, for example,
 * 'T' instead of '10' or 'Q' instead of '12'
 */
impl<'a> fmt::Display for Hand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..consts::HAND_SIZE {
            match self.cards[i].value {
                CardValue::ValueT => {
                    write!(f, "T").unwrap();
                }
                CardValue::ValueJ => {
                    write!(f, "J").unwrap();
                }
                CardValue::ValueQ => {
                    write!(f, "Q").unwrap();
                }
                CardValue::ValueK => {
                    write!(f, "K").unwrap();
                }
                CardValue::ValueA => {
                    write!(f, "A").unwrap();
                }
                _ => {
                    write!(f, "{}", self.cards[i].value.clone() as u8).unwrap();
                }
            }
        }

        write!(f, "")
    }
}

impl<'a> Hand<'a> {
    /**
     * new creates a new Hand from 5 cards from a Table and 2 from a Hole. It finds the best
     * hand for the given cards.
     */
    pub fn new(hole: types::Hole<'a>, table: types::Table<'a>) -> Hand<'a> {
        if let Some(hand) = get_pair(hole, table) {
            hand
        } else {
            check_high_card(hole, table)
        }
    }

    /**
     * get_hand_type gets a string represening the type of a hand
     */
    pub fn get_hand_type(self: &Hand<'a>) -> &'static str {
        match self.hand_type {
            HandType::StraightFlush => "STRAIGHT_FLUSH",
            HandType::FourOfAKind => "FOUR_OF_A_KIND",
            HandType::FullHouse => "FULL_HOUSE",
            HandType::Flush => "FLUSH",
            HandType::Straight => "STRAIGHT",
            HandType::ThreeOfAKind => "THREE_OF_A_KIND",
            HandType::TwoPair => "TWO_PAIR",
            HandType::Pair => "PAIR",
            HandType::HighCard => "HIGH_CARD",
        }
    }
}

/**
 * get_pair finds the best pair on a combination of 5 cards from a Table and 2 from a Hole. It does
 * not check if there is a better hand. If a Pair is not found None is returned.
 */
fn get_pair<'a>(hole: types::Hole<'a>, table: types::Table<'a>) -> Option<Hand<'a>> {
    // Compose a vector with the hole cards and the table cards
    let mut cards: Vec<&Card> = Vec::new();
    for card in hole.iter() {
        cards.push(card);
    }
    for card in table.iter() {
        cards.push(card);
    }
    cards.sort(); // Order the cards so it's possible to know where the most valuable cards are

    // Count the occurrences of each value
    let ref mut occurrences: HashMap<&CardValue, Vec<&Card>> = HashMap::new();
    for card in &cards {
        let count = occurrences.entry(&card.value).or_insert(Vec::new());
        count.push(card);
    }

    let mut hand: Vec<&Card> = Vec::new(); // hand to store the results
    let mut kickers: Vec<&Card> = Vec::new(); // used to check the kicker cards

    // Find the best pair
    let mut high_value = &CardValue::None;
    for (value, cards) in occurrences.iter() {
        if cards.len() == 2 && *value > high_value {
            high_value = *value;
        }
    }

    // Insert the best pair on the hand
    if let Some(pair) = occurrences.get_mut(high_value) {
        for _ in 0..consts::HOLE_SIZE {
            hand.push(pair.pop().unwrap());
        }
    } else {
        return None;
    }

    // Remove the best pair from the occurrences and add the remaining cards into kickers vector
    occurrences.remove(high_value);
    for (_, cards) in occurrences.iter_mut() {
        for card in cards {
            kickers.push(card);
        }
    }

    // Sort the kickers vectors and add the first (higher values) cards to the hand along with
    // the pair
    kickers.sort();
    for _ in 0..consts::HAND_SIZE - consts::HOLE_SIZE {
        hand.push(kickers.pop().unwrap());
    }

    Some(Hand {
        cards: [hand[0], hand[1], hand[2], hand[3], hand[4]],
        hand_type: HandType::Pair,
    })
}

/**
 * get_pair finds the best pair on a combination of 5 cards from a Table and 2 from a Hole. It does
 * not check if there is a better hand. If a Pair is not found None is returned.
 */
fn check_high_card<'a>(hole: types::Hole<'a>, table: types::Table<'a>) -> Hand<'a> {
    // Compose a vector with the hole cards and the table cards
    let mut cards: Vec<&Card> = Vec::new();
    for card in hole.iter() {
        cards.push(card);
    }
    for card in table.iter() {
        cards.push(card);
    }

    cards.sort();
    cards.reverse();

    Hand {
        cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
        hand_type: HandType::HighCard,
    }
}

#[cfg(test)]
mod tests {
    use super::Hand;
    use super::HandType;
    use super::get_pair;
    use super::check_high_card;
    use super::super::card::Card;
    use super::super::card::CardValue;
    use super::super::card::CardSuit;

    #[test]
    #[ignore]
    fn check_high_card_value() {
        let hole = [&Card {
                        value: CardValue::Value3,
                        suit: CardSuit::Spades,
                    },
                    &Card {
                        value: CardValue::Value4,
                        suit: CardSuit::Spades,
                    }];
        let table = [&Card {
                         value: CardValue::Value5,
                         suit: CardSuit::Spades,
                     },
                     &Card {
                         value: CardValue::Value8,
                         suit: CardSuit::Spades,
                     },
                     &Card {
                         value: CardValue::ValueQ,
                         suit: CardSuit::Spades,
                     },
                     &Card {
                         value: CardValue::Value2,
                         suit: CardSuit::Spades,
                     },
                     &Card {
                         value: CardValue::ValueA,
                         suit: CardSuit::Spades,
                     }];

        let hand: Hand = check_high_card(hole, table);

        assert!(hand.hand_type == HandType::HighCard);
        assert!(hand.cards[0].value == CardValue::ValueA);
        assert!(hand.cards[1].value == CardValue::ValueQ);
        assert!(hand.cards[2].value == CardValue::Value8);
        assert!(hand.cards[3].value == CardValue::Value5);
        assert!(hand.cards[4].value == CardValue::Value4);
    }

    #[test]
    #[ignore]
    fn test_get_pair_found() {
        let hole = [&Card {
                        value: CardValue::Value3,
                        suit: CardSuit::Spades,
                    },
                    &Card {
                        value: CardValue::Value3,
                        suit: CardSuit::Spades,
                    }];
        let table = [&Card {
                         value: CardValue::Value3,
                         suit: CardSuit::Spades,
                     },
                     &Card {
                         value: CardValue::Value5,
                         suit: CardSuit::Spades,
                     },
                     &Card {
                         value: CardValue::ValueQ,
                         suit: CardSuit::Spades,
                     },
                     &Card {
                         value: CardValue::Value5,
                         suit: CardSuit::Spades,
                     },
                     &Card {
                         value: CardValue::ValueQ,
                         suit: CardSuit::Spades,
                     }];

        let hand: Hand = get_pair(hole, table).unwrap();

        assert!(hand.hand_type == HandType::Pair);
        assert!(hand.cards[0].value == CardValue::ValueQ);
        assert!(hand.cards[1].value == CardValue::ValueQ);
        assert!(hand.cards[2].value == CardValue::Value5);
        assert!(hand.cards[3].value == CardValue::Value5);
        assert!(hand.cards[4].value == CardValue::Value3);
    }

    #[test]
    #[ignore]
    fn test_get_pair_missing() {
        let hole = [&Card {
                        value: CardValue::Value3,
                        suit: CardSuit::Spades,
                    },
                    &Card {
                        value: CardValue::Value4,
                        suit: CardSuit::Spades,
                    }];
        let table = [&Card {
                         value: CardValue::Value6,
                         suit: CardSuit::Spades,
                     },
                     &Card {
                         value: CardValue::Value7,
                         suit: CardSuit::Spades,
                     },
                     &Card {
                         value: CardValue::ValueQ,
                         suit: CardSuit::Spades,
                     },
                     &Card {
                         value: CardValue::Value5,
                         suit: CardSuit::Spades,
                     },
                     &Card {
                         value: CardValue::ValueA,
                         suit: CardSuit::Spades,
                     }];

        assert_eq!(get_pair(hole, table).is_none(), true);
    }
}

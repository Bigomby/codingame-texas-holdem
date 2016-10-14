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

/**
 * CardValue represents any of the possible values of a Card
 */
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CardValue {
    None,
    Value2 = 2,
    Value3,
    Value4,
    Value5,
    Value6,
    Value7,
    Value8,
    Value9,
    ValueT,
    ValueJ,
    ValueQ,
    ValueK,
    ValueA,
}

/**
 * CardValue represents any of the possible suits of a Card
 */
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardSuit {
    None,
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/**
 * A Card has a value (numeric value) and a suit
 */
#[derive(Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Card {
    pub value: CardValue,
    pub suit: CardSuit,
}

impl Card {
    /**
     * Creates a new instance of a Card given the value and the suit as chars
     */
    pub fn new(value_char: char, suit_char: char) -> Card {
        let value: CardValue;
        let suit: CardSuit;

        // Parse value of the card
        match value_char {
            '2' => value = CardValue::Value2,
            '3' => value = CardValue::Value3,
            '4' => value = CardValue::Value4,
            '5' => value = CardValue::Value5,
            '6' => value = CardValue::Value6,
            '7' => value = CardValue::Value7,
            '8' => value = CardValue::Value8,
            '9' => value = CardValue::Value9,
            'T' => value = CardValue::ValueT,
            'J' => value = CardValue::ValueJ,
            'Q' => value = CardValue::ValueQ,
            'K' => value = CardValue::ValueK,
            'A' => value = CardValue::ValueA,
            _ => {
                println!("Unknown value");
                value = CardValue::None;
            }
        }

        // Parse suit of the card
        match suit_char {
            'C' => suit = CardSuit::Clubs,
            'D' => suit = CardSuit::Diamonds,
            'H' => suit = CardSuit::Hearts,
            'S' => suit = CardSuit::Spades,
            _ => {
                println!("Unknown suit");
                suit = CardSuit::None;
            }
        }

        Card {
            value: value,
            suit: suit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CardValue;
    use super::CardSuit;
    use super::Card;

    #[test]
    #[ignore]
    fn test_new_card() {
        let card1 = Card::new('8', 'D');
        assert!(card1.value == CardValue::Value8);
        assert!(card1.suit == CardSuit::Diamonds);

        let card2 = Card::new('7', 'C');
        assert!(card2.value == CardValue::Value7);
        assert!(card2.suit == CardSuit::Clubs);
    }
}

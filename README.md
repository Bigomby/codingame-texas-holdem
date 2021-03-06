[![Build Status](https://travis-ci.org/Bigomby/codingame-texas-holdem.svg?branch=master)](https://travis-ci.org/Bigomby/codingame-texas-holdem)

# Codingame Texas Holdem

This is my solution for the Codingame Texas Holdem game.

## Goal

You must determine the winner of a hand in Texas Holdem Poker. Output the
winning player, their hand type and the five winning card values.

### Cards

A card is represented by two characters, one for the value and one for the suit.

- `values` (descending order): `A`, `K`, `Q`, `J`, `T`, `9`, `8`, `7`, `6`, `5`,
`4`, `3`, `2`
- `suits` (no order): `C`, `D`, `H`, `S`

### Hands

A player's hand is the best 5 card-card combination from the 7 card combination
of their own 2 hole cards with the 5 community cards. A hand is one of the
following types, in descending order of value:

- `STRAIGHT_FLUSH` - 5 consecutive cards of the same suit
- `FOUR_OF_A_KIND` - 4 cards of matching values and 1 kicker*
- `FULL_HOUSE` - 3 cards of matching values, and 2 of a different matching value
- `FLUSH` - 5 non-consecutive cards, all the same suit
- `STRAIGHT` - 5 consecutive cards
- `THREE_OF_A_KIND` - 3 cards of matching values and 2 kickers*
- `TWO_PAIR` - 2 cards of a matching value, 2 of a different matching value and
  1 kicker*
- `PAIR` - 2 cards of matching values and 3 kickers*, e.g. `66T42`
- `HIGH_CARD` - The absence of any better hand means that the highest value card
  counts with 4 kickers, e.g. `Q7432`

_A kicker is a leftover card which does not contribute to determining the type
of a hand, but is used to break ties. The highest kicker is considered first,
then the next, and so on. Only if none of the kickers are larger might a hand
be considered a `DRAW`. For convenience, we call cards in the 5 card combination
that are not kicker cards, "non-kickers"._

Note that there's no need to implement a special case for a royal flush.

### Determining Winning Hands

A higher HandType beats all lower hand types. If the HandType is the same:

- Place cards in the order given below
- Iterate each card in turn, e.g. 1st card Player 1 vs 1st card player 2
- If one value is higher then that player is the winner, else continue to the
next card
- If no such win is found, the hand is a `DRAW`

### Ordering Cards

Cards are ordered by descending non-kicker values, followed by descending kicker
values. In case of a `FULL_HOUSE`, list the set of 3 before the pair. If an Ace
counts a low in determining a hand, this is also represented in the ordering.

### Worked Example

- Line 1: `TC` `JC`
- Line 2: `AD` `4S`
- Line 3: `2H` `7H` `TH` `QS` `KC`


- Player 1's 7-card hand: `TC` `JC` + `2H` `7H` `TH` `QS` `KC`.
- Player 2's 7-card hand: `AD` `4S` + `2H` `7H` `TH` `QS` `KC`.


- Player 1's best 5-card hand is a `PAIR` of tens (`TC` & `TH`),
and the kickers are `KC`, `QS` & `JC`.
- Player 2's best 5-card hand is a `HIGH_CARD` (`AD`), and the kickers are
`KC`, `QS`, `TH` & `7H`.

A `PAIR` beats a `HIGH_CARD` so Player 1 wins. Thus the output would be:

```
1 PAIR TTKQJ
```

### Input

- Line 1: `holeCardsPlayer1`, the 2 hole cards for Player 1, separated by a
space.
- Line 2: `holeCardsPlayer2`, the 2 hole cards for Player 2, separated by a
space.
- Line 3: `communityCards`, the 5 cards on the board, separated by a space.

### Output

If there's a winner, the output is `WinningPlayerId` `HandType`
`OrderedFiveCardHand`, space-separated. If not, the output is simply `DRAW`.

`WinningPlayerId` is 1 or 2.

`HandType` is as specified above.

`OrderedFiveCardHand` is the winning 5 cards combination's values, in the order
specified above.

Example output:

- `1 HIGH_CARD AKQ72`
- `2 PAIR 55AKQ`
- `1 FULL_HOUSE 22233`
- `DRAW`
- `2 FLUSH 97643`
- `1 STRAIGHT 76543`

### Constraints

All deals are valid. (all cards exist, no duplicate cards are dealt)

### Example

```
Input
8D 7C
7D 6C
KS 9D 5C 3S 2D
Output
1 HIGH_CARD K9875
```

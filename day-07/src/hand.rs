use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::str::FromStr;
use crate::card::{Card, InvalidCardErr};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq)]
pub struct Hand {
    cards: [Card; 5],
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl FromStr for Hand {
    type Err = InvalidCardErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand {
            cards: [
                s[0..1].parse()?,
                s[1..2].parse()?,
                s[2..3].parse()?,
                s[3..4].parse()?,
                s[4..5].parse()?,
            ]
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => Some(self.cards.cmp(&other.cards)),
            ord => Some(ord)
        }
    }
}

impl Hand {
    pub fn hand_type(&self) -> HandType {
        let mut counts: BTreeMap<Card, u8> = BTreeMap::new();
        let mut jokers = 0;

        for card in self.cards {
            if card == Card::Joker {
                jokers += 1;
                continue;
            }

            if let Some(item) = counts.get_mut(&card) {
                *item += 1;
            } else {
                counts.insert(card, 1);
            }
        }

        counts.iter_mut()
            .filter(|(card, _)| *card != &Card::Joker)
            .for_each(|(_, cnt)| *cnt += jokers);

        let mut values = counts.values().collect::<Vec<&u8>>();
        values.sort_by(|a, b| b.cmp(a));

        if values.is_empty() {
            return HandType::FiveOfAKind; // of jokers
        }

        match (values.len(), values[0]) {
            (1, 5) => HandType::FiveOfAKind,
            (2, 4) => HandType::FourOfAKind,
            (2, 3) => HandType::FullHouse,
            (3, 3) => HandType::ThreeOfAKind,
            (3, 2) => HandType::TwoPair,
            (4, 2) => HandType::OnePair,
            (5, 1) => HandType::HighCard,
            err => panic!("Impossible {:?} !", err)
        }
    }
}
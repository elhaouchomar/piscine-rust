use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use rand::Rng;
use rand::thread_rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamand,
    Spade,
    Club,
}



#[derive(Debug, PartialEq)]
pub enum Rank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Suit {
    pub fn random() -> Suit {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let nanos =  now.subsec_nanos();

        match nanos % 4 {
            0 => Suit::Heart,
            1 => Suit::Diamand,
            2 => Suit::Spade,
            _ => Suit::Club,
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamand,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
    }
}

impl Rank {
pub fn random() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let nanos = now.subsec_nanos();
        
        match nanos % 5 {
            1 => Rank::Jack,
            2 => Rank::Queen,
            3 => Rank::King,
        4 => {
        let mut randame = thread_rng();
        let nb = randame.gen_range(2..=10);
        return Rank::Number(nb);
    },
            _ => Rank::Ace,
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            v @ 2..=10 => Rank::Number(v),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Ace,
        }

    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit  == Suit::Spade && card.rank == Rank::Ace
}
use rand::{self, rng, seq::SliceRandom};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Suit {
    Diamonds,
    Hearts,
    Clubs,
    Spades,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn value(self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        use Rank::*;
        use Suit::*;

        let mut cards = Vec::with_capacity(44);

        let all_ranks = [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ];

        for &suit in &[Diamonds, Hearts, Clubs, Spades] {
            for &rank in &all_ranks {
                let is_red = matches!(suit, Diamonds | Hearts);
                let is_face_or_ace = matches!(rank, Jack | Queen | King | Ace);

                if is_red && is_face_or_ace {
                    continue;
                }
                cards.push(Card::new(suit, rank));
            }
        }

        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut prng = rng();
        self.cards.shuffle(&mut prng);
    }

    pub fn deal(&mut self, n: usize) -> Vec<Card> {
        let len = self.cards.len();
        if n >= len {
            self.cards.drain(..).collect()
        } else {
            self.cards.split_off(len - n)
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

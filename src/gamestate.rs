use crate::card::{Card, Deck, Suit};

const ROOM_SIZE: usize = 4;
pub const MAX_HEALTH: u8 = 20;

pub struct Gamestate {
    deck: Deck,
    graveyard: Vec<Card>,
    pub room: Vec<Card>,
    pub weapon: Option<Card>,
    pub weapon_slain: Vec<Card>,
    pub player_health: u8,
    pub history: Vec<PlayerAction>,
    turns_since_reset: usize,
}

impl Gamestate {
    pub fn new(deck: Deck) -> Self {
        let mut init = Self {
            deck,
            graveyard: Vec::new(),
            room: Vec::new(),
            weapon: None,
            weapon_slain: Vec::new(),
            player_health: MAX_HEALTH,
            history: Vec::new(),
            turns_since_reset: 0,
        };
        init.fill_room();
        init
    }

    // UI access
    pub fn view<'a>(&'a self) -> GameView<'a> {
        GameView {
            room: &self.room,
            weapon: self.weapon,
            weapon_slain: &self.weapon_slain,
            player_health: self.player_health,
            deck_remaining: self.deck.len(),
        }
    }

    pub fn is_ongoing(&self) -> bool {
        self.is_alive() && !self.deck.is_empty() && !self.room.is_empty()
    }

    // Gameplay methods
    pub fn legal_actions(&self) -> Vec<PlayerAction> {
        let mut actions = Vec::new();

        for (i, card) in self.room.iter().enumerate() {
            match card.suit {
                Suit::Clubs | Suit::Spades => {
                    actions.push(PlayerAction::Fight {
                        index: i,
                        mode: FightMode::Barehanded,
                    });

                    // Check if fighting with weapon is legal
                    if self.can_fight_with_weapon(card) {
                        actions.push(PlayerAction::Fight {
                            index: i,
                            mode: FightMode::WithWeapon,
                        })
                    }
                }
                Suit::Hearts => {
                    if self.can_use_potion() {
                        actions.push(PlayerAction::Potion(i));
                    }
                }
                Suit::Diamonds => {
                    actions.push(PlayerAction::Weapon(i));
                }
            }
        }

        if !matches!(self.history.last(), Some(PlayerAction::Flee)) && self.room.len() == 4 {
            actions.push(PlayerAction::Flee);
        }

        actions
    }

    pub fn resolve_action(&mut self, action: PlayerAction) {
        use FightMode::*;
        use PlayerAction::*;

        match action {
            Potion(index) => self.resolve_potion(index),
            Weapon(index) => self.resolve_weapon(index),
            Fight { index, mode } => match mode {
                Barehanded => self.resolve_fight_barehanded(index),
                WithWeapon => self.resolve_fight_with_weapon(index),
            },
            Flee => {
                self.deck.cards.splice(0..0, self.room.drain(..));
            }
        }
        self.history.push(action);
        self.turns_since_reset += 1;

        if self.room.len() <= 1 {
            self.fill_room();
            self.turns_since_reset = 0;
        }
    }

    pub fn compute_score(&self) -> isize {
        if self.is_alive() {
            return self.player_health as isize;
        }

        let room_score: isize = self
            .room
            .iter()
            .map(|x| match x.suit {
                Suit::Clubs | Suit::Spades => x.rank.value() as isize,
                _ => 0,
            })
            .sum();

        let deck_score: isize = self
            .deck
            .cards
            .iter()
            .map(|x| match x.suit {
                Suit::Clubs | Suit::Spades => x.rank.value() as isize,
                _ => 0,
            })
            .sum();

        -deck_score - room_score
    }

    // Private helpers
    fn fill_room(&mut self) {
        let space = ROOM_SIZE.saturating_sub(self.room.len());
        if space == 0 {
            return;
        }

        let new_cards = self.deck.deal(space);
        self.room.extend(new_cards);
    }

    fn take_damage(&mut self, amount: u8) {
        self.player_health = self.player_health.saturating_sub(amount);
    }

    fn is_alive(&self) -> bool {
        self.player_health > 0
    }

    fn use_potion(&mut self, potion: Card) {
        self.player_health = (self.player_health + potion.rank.value()).min(MAX_HEALTH);
        self.graveyard.push(potion);
    }

    fn equip_weapon(&mut self, weapon: Card) {
        self.graveyard.extend(self.weapon_slain.drain(..));
        self.weapon_slain.clear();
        if let Some(old_weapon) = self.weapon.take() {
            self.graveyard.push(old_weapon);
        }
        self.weapon = Some(weapon);
    }

    fn can_fight_with_weapon(&self, monster: &Card) -> bool {
        if self.weapon.is_some() {
            if let Some(slain) = self.weapon_slain.last() {
                return slain.rank.value() >= monster.rank.value();
            }
            return true;
        }
        false
    }

    fn can_use_potion(&self) -> bool {
        if self.room.iter().all(|c| matches!(c.suit, Suit::Hearts)) {
            return true;
        }
        let start_index = self.history.len().saturating_sub(self.turns_since_reset);
        let potion_used_in_room = self.history[start_index..]
            .iter()
            .any(|a| matches!(a, PlayerAction::Potion(_)));
        !potion_used_in_room
    }

    // Resolution helpers
    fn resolve_potion(&mut self, index: usize) {
        let card = self.room.remove(index);
        self.use_potion(card);
    }

    fn resolve_weapon(&mut self, index: usize) {
        let card = self.room.remove(index);
        self.equip_weapon(card);
    }

    fn resolve_fight_barehanded(&mut self, index: usize) {
        let card = self.room.remove(index);
        self.take_damage(card.rank.value());
        self.graveyard.push(card);
    }

    fn resolve_fight_with_weapon(&mut self, index: usize) {
        let card = self.room.remove(index);
        if !self.can_fight_with_weapon(&card) {
            self.take_damage(card.rank.value());
            self.graveyard.push(card);
            return;
        }

        if let Some(weapon) = self.weapon {
            let damage = card.rank.value().saturating_sub(weapon.rank.value());
            if damage >= self.player_health {
                // Player dies
                self.player_health = 0;
                self.graveyard.push(card);
                self.graveyard.extend(self.weapon_slain.drain(..));
                self.graveyard.push(weapon);
                self.weapon = None;
            } else {
                self.take_damage(damage);
                self.weapon_slain.push(card);
            }
        }
    }
}

// Read-only view of the game for UI rendering.
pub struct GameView<'a> {
    pub room: &'a [Card],
    pub weapon: Option<Card>,
    pub weapon_slain: &'a [Card],
    pub player_health: u8,
    pub deck_remaining: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FightMode {
    Barehanded,
    WithWeapon,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerAction {
    Potion(usize),
    Weapon(usize),
    Fight { index: usize, mode: FightMode },
    Flee,
}

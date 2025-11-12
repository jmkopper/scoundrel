use crate::card::{Card, Rank, Suit};
use crate::gamestate::{FightMode, GameView, MAX_HEALTH, PlayerAction};
use colored::*;
use std::io::{self, Write};

const INPUT_ERR: usize = 999;

pub trait GameUI {
    fn render(&self, view: &GameView);
    fn get_action(&self, gamestate: &GameView, legal_actions: &[PlayerAction]) -> PlayerAction;
    fn render_game_over(&self, gamestate: &GameView, score: isize);
}

pub struct TextUI;

impl TextUI {
    fn card_to_string(&self, card: Card) -> ColoredString {
        let (suit, color) = match card.suit {
            Suit::Clubs => ("♣", "bright black"),
            Suit::Diamonds => ("♦", "bright red"),
            Suit::Hearts => ("♥", "bright red"),
            Suit::Spades => ("♠", "bright black"),
        };

        let rank = match card.rank {
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
            Rank::Ace => "Ace",
        };

        return format!("{}{}", suit, rank).color(color).bold();
    }

    fn action_to_string(&self, action: PlayerAction, view: &GameView) -> String {
        match action {
            PlayerAction::Fight { index, mode } => {
                let card_str = self.card_to_string(view.room[index]);
                match mode {
                    FightMode::Barehanded => return format!("Fight {} barehanded", card_str),
                    FightMode::WithWeapon => return format!("Fight {} with weapon", card_str),
                }
            }
            PlayerAction::Potion(i) => {
                format!("Use potion: {}", self.card_to_string(view.room[i]))
            }
            PlayerAction::Weapon(i) => {
                format!("Equip weapon: {}", self.card_to_string(view.room[i]))
            }
            PlayerAction::Flee => "Flee!".to_string(),
        }
    }
}

impl GameUI for TextUI {
    fn render(&self, view: &GameView) {
        print!("\n============ Current room ============\n  ");
        for card in view.room.iter() {
            print!("{}    ", self.card_to_string(*card));
        }
        println!("\n======================================\n");
        io::stdout().flush().unwrap();

        println!("Cards in deck: {}", view.deck_remaining);
        println!(
            "Health: {}/{}",
            format!("{}", view.player_health).bright_cyan(),
            format!("{}", MAX_HEALTH).bright_cyan()
        );
        if let Some(weapon) = view.weapon {
            println!("Weapon: {}", self.card_to_string(weapon));
            for card in view.weapon_slain.iter() {
                println!("        {}", self.card_to_string(*card));
            }
        }
        println!();
    }

    fn get_action(&self, view: &GameView, legal_actions: &[PlayerAction]) -> PlayerAction {
        let mut action: Option<PlayerAction> = None;

        for (i, a) in legal_actions.iter().enumerate() {
            let i_string = format!("{}", i + 1).blue();
            println!("[{}] - {}", i_string, self.action_to_string(*a, view))
        }

        while action.is_none() {
            print!("\nChoose action: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let mut choice: usize = input.trim().parse().unwrap_or(INPUT_ERR);
            choice = choice.saturating_sub(1);
            if choice >= legal_actions.len() {
                println!("Invalid input!");
                continue;
            }
            action = Some(legal_actions[choice]);
        }

        action.unwrap()
    }

    fn render_game_over(&self, view: &GameView, score: isize) {
        println!("\n=== Game over ===");
        println!("Final health: {}/{}", view.player_health, MAX_HEALTH);
        let score_text = format!("{}", score).bold().yellow();
        println!("Your score: {}", score_text);
        println!("=================\n")
    }
}

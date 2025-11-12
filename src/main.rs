use crate::{card::Deck, gamestate::Gamestate, ui::GameUI};

mod card;
mod gamestate;
mod ui;

fn main() {
    let tui = ui::TextUI {};
    let mut deck = Deck::new();
    deck.shuffle();
    let mut gamestate = Gamestate::new(deck);

    while gamestate.is_ongoing() {
        let view = gamestate.view();
        tui.render(&view);

        let legal_actions = gamestate.legal_actions();
        let action = tui.get_action(&view, &legal_actions);
        gamestate.resolve_action(action);
    }

    let view = gamestate.view();
    let score = gamestate.compute_score();
    tui.render_game_over(&view, score);
}

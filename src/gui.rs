use std::io::BufRead;

use crate::trade_good::TradeGood;

pub enum Action {
    Buy(TradeGood, usize),
    Sell(TradeGood, usize),
    ListPlanets,
    Move(usize),
    Quit,
}

pub fn get_next_input_line() -> String {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    lines.next().unwrap().unwrap()
}

pub fn prompt(question: &str) -> String {
    println!("{}", question);
    get_next_input_line()
}

pub fn get_next_action() -> Action {
    let mut input = None;
    while input.is_none() {
        let next_line = prompt("What would you like to do?");
        input = match next_line.as_ref() {
            "buy" => Some(Action::Buy(TradeGood::Iron, 5)),
            "sell" => Some(Action::Sell(TradeGood::Iron, 5)),
            "move" => Some(Action::Move(10)),
            "list planets" => Some(Action::ListPlanets),
            "list" => Some(Action::ListPlanets),
            "quit" => Some(Action::Quit),
            _ => {
                println!("Invalid input");
                None
            }
        }
    }
    input.unwrap()
}

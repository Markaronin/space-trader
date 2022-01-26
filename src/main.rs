use consts::*;
use gui::prompt;
use std::collections::BTreeMap;
use trade_good::*;

mod consts;
mod gui;
pub mod trade_good;

struct Planet {
    name: String,
    prices: BTreeMap<TradeGood, usize>,
}
impl Planet {
    fn new(name: String) -> Self {
        Planet {
            name,
            prices: TradeGood::default_prices(),
        }
    }

    fn show(&self) {
        println!("Planet {}", self.name);
        for (key, value) in &self.prices {
            println!("\t{:?}: {}", key, value);
        }
    }
}
struct Planets {
    data: Vec<Planet>,
}
impl Planets {
    fn new(num_planets: usize) -> Self {
        Planets {
            data: (0..num_planets)
                .map(|i| Planet::new(i.to_string()))
                .collect::<Vec<_>>(),
        }
    }

    fn list(&self) {
        self.data.iter().for_each(|planet| planet.show())
    }
}

struct Player {
    name: String,
    money: usize,
    location: usize,
}
impl Player {
    fn new(name: String) -> Self {
        Player {
            name,
            money: STARTING_PLAYER_MONEY,
            location: 0,
        }
    }
}

fn main() {
    let mut planets = Planets::new(NUM_PLANETS);
    let mut player = Player::new(prompt("What is your name?"));

    loop {
        let action = gui::get_next_action();
        match action {
            gui::Action::Buy(trade_good, amount) => println!("Trading"),
            gui::Action::Sell(trade_good, amount) => println!("Trading"),
            gui::Action::Move(new_planet) => {
                player.location = new_planet;
                println!("Moved to {new_planet}");
            }
            gui::Action::ListPlanets => {
                planets.list();
            }
            gui::Action::Quit => break,
        }
    }
}

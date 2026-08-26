// main

mod game;
mod input_helper;
use crate::game::Domain;
use game::Game;

fn main() {
    let mut game = set_start_params();

    while let Some(current_example) = &game.current {
        let user_anser: f32 = input_helper::input(&format!(
            "Enter answer  {} : ",
            current_example.text_example
        ));

        let result = game.check_answer(user_anser);

        match result {
            game::Respond::Answer(answer_type) => {
                println!("Answer status: {:?}", answer_type);
            }
            game::Respond::Finished(stats) => {
                println!("Finished!\n{:#?}", stats);
                break;
            }
        }
    }
}

fn set_start_params() -> Game {
    let domain1: String = input_helper::input(r#"Enter first doimain like "first second" : "#);
    let domain2: String = input_helper::input(r#"Enter second doimain like "first second" : "#);
    let amount: usize = input_helper::input("Enter amount: ");
    let firs_domain = get_domain(&domain1);
    let second_domain = get_domain(&domain2);
    let game = Game::new(firs_domain, second_domain, amount);
    game
}
fn get_domain(str: &str) -> Domain {
    let parts: Vec<&str> = str.split_whitespace().collect();
    Domain {
        from: parts.first().unwrap().parse().unwrap(),
        to: parts.get(1).unwrap().parse().unwrap(),
    }
}

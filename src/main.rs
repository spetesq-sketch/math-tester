// main

mod game;
mod input_helper;
mod stats;

use crate::game::{Domain, ExampleType};
use crossterm::style::Stylize;
use game::AnswerType;
use game::Game;

fn main() {
    let mut game = set_start_params();
    let mut history = Vec::new();

    while let Some(current_example) = &game.current {
        print!("History: ");
        for symbol in &history {
            print!("{}", symbol);
        }
        println!();
        let user_answer: f32 = input_helper::input(&format!(
            "Enter answer  {} : ",
            current_example.text_example
        ));

        let result = game.check_answer(user_answer);
        match result {
            game::Respond::Answer(answer_type) => {
                println!("Answer status: {:?}", answer_type);
                match answer_type.answer_type {
                    AnswerType::Correct => history.push('█'.green()),
                    AnswerType::Wrong(_) => history.push('█'.red()),
                }
            }
            game::Respond::Finished(stats) => {
                println!("Finished!\n{:#?}", stats);
                break;
            }
        }
    }
}

fn set_start_params() -> Game {
    let domain1: String =
        input_helper::input(r#"Enter first domain like "first second" like "0 100": "#);
    let domain2: String =
        input_helper::input(r#"Enter second domain like "first second" like "0 100": "#);
    let example_types: Vec<usize> = input_helper::input::<String>(
        "Enter types 1 - Addition 2 - Subtraction 3 - Multiplication 4 - Division: ",
    )
    .split_whitespace()
    .map(|s| s.parse().expect("Please enter valid numbers"))
    .collect();

    let mut digit: String =
        input_helper::input("Enter number of digit after point (default = 2): ");
    let mut final_digit = 2usize;
    if let Ok(final_) = digit.parse::<usize>() {
        final_digit = final_;
    }

    let mut a = Vec::new();
    for i in example_types {
        let result = match i {
            1 => ExampleType::Addiction,
            2 => ExampleType::Subtraction,
            3 => ExampleType::Multiplication,
            4 => ExampleType::Division,
            _ => panic!(),
        };
        a.push(result)
    }
    let amount: usize = input_helper::input("Enter amount: ");
    let firs_domain = get_domain(&domain1);
    let second_domain = get_domain(&domain2);

    Game::new(firs_domain, second_domain, amount, a, final_digit)
}
fn get_domain(str: &str) -> Domain {
    let parts: Vec<&str> = str.split_whitespace().collect();
    Domain {
        from: parts.first().unwrap().parse().unwrap(),
        to: parts.get(1).unwrap().parse().unwrap(),
    }
}

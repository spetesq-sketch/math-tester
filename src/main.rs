// main
mod game;
mod input_helper;
mod stats;

use crate::game::{Domain, ExampleType};
use clap::builder::StyledStr;
use crossterm::style::{StyledContent, Stylize};
use game::AnswerType;
use game::Game;

fn main() {
    let mut game = set_start_params();
    let mut history = Vec::new();
    while let Some(current_example) = game.get_current() {
        print_history(&history);
        let user_answer: f32 = input_helper::input(&format!(
            "Enter answer  {} : ",
            current_example.text_example
        ));
        let result = game.check_answer(user_answer);
        match result {
            game::Respond::Answer(answer_result) => {
                println!("Answer status: {:?}", answer_result);
                match answer_result.answer_type {
                    AnswerType::Correct => history.push('█'.green()),
                    AnswerType::Wrong { .. } => history.push('█'.red()),
                }
            }
            game::Respond::Finished(stats) => {
                print_history(&history);
                println!("Finished!\n{:#?}", stats);
                break;
            }
        }
    }
}

fn set_start_params() -> Game {
    let first_domain_input: String =
        input_helper::input(r#"Enter first domain like "first second" like "0 100": "#);
    let second_domain_input: String =
        input_helper::input(r#"Enter second domain like "first second" like "0 100": "#);
    let example_type_choices: Vec<usize> = input_helper::input::<String>(
        "Enter types 1 - Addition 2 - Subtraction 3 - Multiplication 4 - Division: ",
    )
    .split_whitespace()
    .map(|s| s.parse().expect("Please enter valid numbers"))
    .collect();

    let digit_input: String =
        input_helper::input("Enter number of digit after point (default = 2): ");
    let mut precision = 2usize;
    if let Ok(parsed_precision) = digit_input.parse::<usize>() {
        precision = parsed_precision;
    }

    let mut selected_types = Vec::new();
    for choice in example_type_choices {
        let example_type = match choice {
            1 => ExampleType::Addition,
            2 => ExampleType::Subtraction,
            3 => ExampleType::Multiplication,
            4 => ExampleType::Division,
            _ => panic!("Unknown example type: {choice}"),
        };
        selected_types.push(example_type)
    }

    let amount: usize = input_helper::input("Enter amount: ");
    let first_domain = get_domain(&first_domain_input);
    let second_domain = get_domain(&second_domain_input);
    Game::new(
        first_domain,
        second_domain,
        amount,
        selected_types,
        precision,
    )
}

fn print_history(history: &Vec<StyledContent<char>>) {
    print!("History: ");
    for symbol in history {
        print!("{}", symbol);
    }
    println!();
}

fn get_domain(input: &str) -> Domain {
    let parts: Vec<&str> = input.split_whitespace().collect();
    Domain {
        from: parts.first().unwrap().parse().unwrap(),
        to: parts.get(1).unwrap().parse().unwrap(),
    }
}

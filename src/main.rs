// main
mod file_manager;
mod game;
mod input_helper;
mod stats;

use crate::game::{Domain, ExampleType, Stats};
use crate::stats::Statistic;
use anyhow::{Context, Result};
use clap::builder::StyledStr;
use clap::{Parser, Subcommand};
use crossterm::style::{StyledContent, Stylize};
use game::AnswerType;
use game::Game;
use std::time::Duration;

#[derive(Parser)]
#[command(author = "lixtr", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the game
    Game,
    /// Show statistic
    Stat,
}

fn main() -> Result<()> {
    let parser = Cli::parse();
    match parser.command {
        Command::Game => run_game_loop()?,
        Command::Stat => {
            let stat = Statistic::load()?;
            print_statistic(stat.total_statistic);
        }
    }
    Ok(())
}

fn print_statistic(stat: Stats) {
    println!(" -- Total Statistic -- ");
    println!("Total examples: {}", &stat.total_amount);
    println!("Total time: {}", format_duration(stat.total_time));
    println!("Total correct: {}", &stat.total_correct);
    println!("Total wrong: {}", &stat.total_wrong);
}
fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();

    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
fn run_game_loop() -> Result<()> {
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
                let mut stat = Statistic::load()?;
                stat.add_stats(&stats);
                stat.save()?;
                println!("Finished!\n{:#?}", stats);
                break;
            }
        }
    }
    Ok(())
}
fn set_start_params() -> Game {
    let first_domain = read_domain(r#"Enter first domain like "0 100": "#);
    let second_domain = read_domain(r#"Enter first domain like "0 100": "#);
    let selected_types = read_example_types(
        "Enter types 1 - Addition 2 - Subtraction 3 - Multiplication 4 - Division: ",
    );
    let precision = read_precision("Enter number of digit after point (default = 2): ");
    let amount = read_amount("Enter amount: ");

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

fn read_valid<T>(prompt: &str, mut parser: impl FnMut(&str) -> Result<T, String>) -> T {
    loop {
        let user_input: String = input_helper::input(prompt);
        match parser(&user_input) {
            Ok(value) => return value,
            Err(message) => println!("Error: {message}"),
        }
    }
}
fn read_domain(promt: &str) -> Domain {
    read_valid(promt, |input| {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(format!("2 numbers required, received: {}", parts.len()));
        }
        let from: u32 = parts[0]
            .parse()
            .map_err(|_| format!("\"{}\" - Not an integer", parts[0]))?;
        let to: u32 = parts[1]
            .parse()
            .map_err(|_| format!("\"{}\" - Not an integer", parts[1]))?;
        if from >= to {
            return Err(format!(
                "The first number ({from}) must be less than the second ({to})"
            ));
        }
        Ok(Domain { from, to })
    })
}

fn read_example_types(prompt: &str) -> Vec<ExampleType> {
    read_valid(prompt, |input| {
        let mut selected_types = Vec::new();
        for token in input.split_whitespace() {
            let choice: usize = token
                .parse()
                .map_err(|_| format!("{} - Not an integer", token))?;
            let example_type = match choice {
                1 => ExampleType::Addition,
                2 => ExampleType::Subtraction,
                3 => ExampleType::Multiplication,
                4 => ExampleType::Division,
                other => return Err(format!("No type with number {}, available 1-4", other)),
            };
            selected_types.push(example_type);
        }
        if selected_types.is_empty() {
            return Err("You must select at least one type".to_string());
        }
        Ok(selected_types)
    })
}
fn read_precision(prompt: &str) -> usize {
    let input: String = input_helper::input(prompt);
    input.trim().parse().unwrap_or(2)
}
fn read_amount(prompt: &str) -> usize {
    read_valid(prompt, |raw| {
        let amount: usize = raw
            .parse()
            .map_err(|_| format!("\"{raw}\" - Not an integer"))?;
        if amount == 0 {
            return Err("The number of examples must be greater than zero".to_string());
        }
        Ok(amount)
    })
}

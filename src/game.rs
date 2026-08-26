// game

use rand::RngExt;
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub amount: usize,
    pub correct: usize,
    pub wrong: usize,
    pub time: Duration,
}
impl Stats {
    fn new(amount: usize) -> Self {
        Self {
            amount,
            correct: 0,
            wrong: 0,
            time: Duration::ZERO,
        }
    }
}

#[derive(Debug)]
pub enum AnswerType {
    Correct,
    Wrong(f32),
}

#[derive(Debug)]
pub struct AnswerThing {
    pub answer_type: AnswerType,
    pub time: Duration,
}

#[derive(Debug)]
pub enum Respond {
    Answer(AnswerThing),
    Finished(Stats),
}

#[derive(Copy, Clone, Debug)]
pub enum ExampleType {
    Addiction,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, Clone)]
pub struct Example {
    pub text_example: String,
    timer: Instant,
    pub time: Duration,
    pub example_type: ExampleType,
    pub answer: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Domain {
    pub from: f32,
    pub to: f32,
}

#[derive(Debug)]
pub struct Game {
    pub domain_first: Domain,
    pub domain_second: Domain,
    pub current: Option<Example>,
    current_index: usize,
    pub amount: usize,
    pub examples: Vec<Example>,
    pub stats: Stats,
    pub digit_after_point: usize,
}

impl Game {
    pub fn new(
        domain_first: Domain,
        domain_second: Domain,
        amount: usize,
        example_types: Vec<ExampleType>,
        digit: usize,
    ) -> Self {
        let mut game = Game {
            domain_first,
            domain_second,
            current: None,
            current_index: 0,
            amount,
            examples: Vec::new(),
            stats: Stats::new(amount),
            digit_after_point: digit,
        };
        game.generate(example_types);
        game.set_current();
        game
    }
    pub fn check_answer(&mut self, answer: f32) -> Respond {
        let (answer_type, elapsed_time) = {
            if let Some(example) = &mut self.current {
                example.time = example.timer.elapsed();
                self.stats.time += example.time;

                let type_of_ans = if answer == example.answer {
                    self.stats.correct += 1;
                    AnswerType::Correct
                } else {
                    self.stats.wrong += 1;
                    AnswerType::Wrong(example.answer)
                };

                (type_of_ans, example.time)
            } else {
                return Respond::Finished(self.stats);
            }
        };

        if !self.next() {
            return Respond::Finished(self.stats);
        }

        Respond::Answer(AnswerThing {
            answer_type,
            time: elapsed_time,
        })
    }

    fn next(&mut self) -> bool {
        self.current_index += 1;
        self.set_current();
        if let Some(example) = &mut self.current {
            example.timer = Instant::now();
            true
        } else {
            false
        }
    }

    fn generate(&mut self, example_types: Vec<ExampleType>) {
        let mut rng = rand::rng();
        for _example in 0..self.amount {
            if let Some(example_type) = example_types.choose(&mut rng) {
                self.examples.push(self.generate_example(*example_type))
            }
        }
    }
    fn set_current(&mut self) {
        self.current = self.examples.get(self.current_index).cloned();
    }

    fn generate_example(&self, example_type: ExampleType) -> Example {
        let mut rng = rand::rng();

        let first_num =
            rng.random_range(self.domain_first.from as u32..self.domain_first.to as u32) as f32;
        let second_num =
            rng.random_range(self.domain_second.from as u32..self.domain_second.to as u32) as f32;

        let operator = match example_type {
            ExampleType::Addiction => '+',
            ExampleType::Subtraction => '-',
            ExampleType::Multiplication => '*',
            ExampleType::Division => '/',
        };

        Example {
            text_example: format!("{} {} {}", first_num, operator, second_num),
            timer: Instant::now(),
            time: Duration::ZERO,
            example_type,
            answer: get_answer(first_num, second_num, example_type, self.digit_after_point),
        }
    }
}
fn get_answer(num1: f32, num2: f32, operator: ExampleType, digit: usize) -> f32 {
    match operator {
        ExampleType::Addiction => num1 + num2,
        ExampleType::Subtraction => num1 - num2,
        ExampleType::Multiplication => num1 * num2,
        ExampleType::Division => {
            let result = num1 / num2;
            let multiplier = 10f32.powi(digit as i32);
            let rounded = (result * multiplier).round() / multiplier;
            rounded
        }
    }
}

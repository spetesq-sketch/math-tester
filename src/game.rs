// game

use rand::RngExt;
use rand::rngs::ThreadRng;
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
    Wrong { correct_answer: f32 },
}

#[derive(Debug)]
pub struct AnswerResult {
    pub answer_type: AnswerType,
    pub time: Duration,
}

#[derive(Debug)]
pub enum Respond {
    Answer(AnswerResult),
    Finished(Stats),
}

#[derive(Copy, Clone, Debug)]
pub enum ExampleType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, Clone)]
pub struct Example {
    pub text_example: String,
    timer: Option<Instant>,
    pub time: Duration,
    pub example_type: ExampleType,
    pub answer: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Domain {
    pub from: u32,
    pub to: u32,
}

#[derive(Debug)]
pub struct Game {
    pub domain_first: Domain,
    pub domain_second: Domain,
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
            current_index: 0,
            amount,
            examples: Vec::new(),
            stats: Stats::new(amount),
            digit_after_point: digit,
        };
        game.generate(example_types);
        game
    }

    pub fn check_answer(&mut self, answer: f32) -> Respond {
        let (is_correct, correct_answer, elapsed_time) = {
            if let Some(example) = self.get_current_mut() {
                if let Some(timer) = example.timer {
                    example.time = timer.elapsed();
                    example.timer = None;
                }
                (answer == example.answer, example.answer, example.time)
            } else {
                return Respond::Finished(self.stats);
            }
        };

        self.stats.time += elapsed_time;
        let resulting_type = if is_correct {
            self.stats.correct += 1;
            AnswerType::Correct
        } else {
            self.stats.wrong += 1;
            AnswerType::Wrong { correct_answer }
        };

        if !self.next() {
            return Respond::Finished(self.stats);
        }

        Respond::Answer(AnswerResult {
            answer_type: resulting_type,
            time: elapsed_time,
        })
    }
    fn next(&mut self) -> bool {
        self.current_index += 1;
        if let Some(example) = self.get_current_mut() {
            example.timer = Some(Instant::now());
            true
        } else {
            false
        }
    }

    fn generate(&mut self, example_types: Vec<ExampleType>) {
        let mut rng = rand::rng();
        for _ in 0..self.amount {
            if let Some(example_type) = example_types.choose(&mut rng) {
                self.examples
                    .push(self.generate_example(*example_type, &mut rng))
            }
        }
    }

    fn get_current_mut(&mut self) -> Option<&mut Example> {
        self.examples.get_mut(self.current_index)
    }

    pub fn get_current(&self) -> Option<&Example> {
        self.examples.get(self.current_index)
    }

    fn generate_example(&self, example_type: ExampleType, rng: &mut ThreadRng) -> Example {
        let first_num = rng.random_range(self.domain_first.from..self.domain_first.to) as f32;
        let second_num = rng.random_range(self.domain_second.from..self.domain_second.to) as f32;

        let operator = match example_type {
            ExampleType::Addition => '+',
            ExampleType::Subtraction => '-',
            ExampleType::Multiplication => '*',
            ExampleType::Division => '/',
        };

        Example {
            text_example: format!("{} {} {}", first_num, operator, second_num),
            timer: None,
            time: Duration::ZERO,
            example_type,
            answer: get_answer(first_num, second_num, example_type, self.digit_after_point),
        }
    }
}

fn get_answer(first_num: f32, second_num: f32, operator: ExampleType, digit: usize) -> f32 {
    match operator {
        ExampleType::Addition => first_num + second_num,
        ExampleType::Subtraction => first_num - second_num,
        ExampleType::Multiplication => first_num * second_num,
        ExampleType::Division => {
            let result = first_num / second_num;
            let multiplier = 10f32.powi(digit as i32);
            let rounded = (result * multiplier).round() / multiplier;
            rounded
        }
    }
}

use crate::guess::Strategy;
use crate::primes::{get_components, get_prime_factors};
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};

mod guess;
mod primes;

const DEFAULT_TARGET: &str = "77";
const DEFAULT_INPUT: &str = "1,2,3,4,5,25";

fn main() -> Result<()> {
    let theme = ColorfulTheme::default();

    let target: u32 = Input::with_theme(&theme)
        .with_prompt("What's the target value?")
        .with_initial_text(DEFAULT_TARGET)
        .interact_text()?;

    let numbers: String = Input::with_theme(&theme)
        .with_prompt("What are the given numbers? (enter comma separated list)")
        .with_initial_text(DEFAULT_INPUT)
        .interact_text()?;
    let numbers = get_numbers(&numbers);

    let strategy: Strategy = dialoguer::Select::with_theme(&theme)
        .with_prompt("Select a strategy")
        .items(&Strategy::all())
        .default(Strategy::default() as usize)
        .interact()?
        .into();

    let prime_factors = get_prime_factors(target);
    let factors = get_components(target);

    println!("\t-> options given: {numbers:?}");
    println!("\t-> prime factors {prime_factors:?}");
    println!("\t-> components {factors:?}");

    let guess = guess::operations(target, numbers, strategy);
    println!("\t-> strategy {strategy:?}");
    println!("\t-> guess {guess:?}");

    Ok(())
}

fn get_numbers(input: &str) -> Vec<u32> {
    let a = input
        .split(',')
        .map(str::trim)
        .filter(|i| !i.is_empty())
        .map(|i| i.parse().unwrap_or_else(|_| panic!("invalid number: {i}")))
        .collect();
    a
}

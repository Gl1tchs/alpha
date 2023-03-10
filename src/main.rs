use clap::{arg, CommandFactory, Parser};
use colored::Colorize;

use alpha::{
    alphabet::{Alphabet, Letter},
    capitalize_first,
};

use std::collections::HashMap;
use strsim::levenshtein;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Letter to return.
    letter: Option<String>,

    /// Should print alphabet table?
    #[arg(short, long)]
    table: bool,

    /// Should return the letter uppercase?
    #[arg(short, long)]
    upper: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let alphabet: Alphabet = Alphabet::import(GREEK_ALPHABET_JSON)?;

    if args.table {
        alphabet.print_table();
        return Ok(());
    }

    match args.letter {
        Some(letter_arg) => {
            let letter: Option<&Letter> = alphabet
                .letters
                .iter()
                .find(|&letter| letter.name == letter_arg);

            match letter {
                Some(letter) => {
                    let letter_str = match args.upper {
                        true => letter.upper.clone(),
                        false => letter.lower.clone(),
                    };

                    println!(
                        "{}: {}",
                        capitalize_first(letter_arg.to_lowercase().as_str()),
                        letter_str
                    );
                }
                None => {
                    let mut suggestions = HashMap::new();
                    let mut min_distance = usize::max_value();
                    for l in &alphabet.letters {
                        let distance = levenshtein(&letter_arg, &l.name);
                        if distance <= 2 {
                            // set a threshold for suggestions
                            suggestions.insert(l.name.clone(), distance);
                            if distance < min_distance {
                                min_distance = distance;
                            }
                        }
                    }

                    if !suggestions.is_empty() {
                        let closest_matches: Vec<String> = suggestions
                            .iter()
                            .filter(|&(_, d)| *d == min_distance)
                            .map(|(l, _)| capitalize_first(l))
                            .collect();
                        let matches_str = closest_matches.join(", ");
                        println!(
                            "{}{}",
                            "Unable to find letter: ".red(),
                            letter_arg.to_lowercase().bold().red()
                        );
                        println!("Did you mean: {}", matches_str.to_lowercase());
                    } else {
                        println!(
                            "{}{}",
                            "Unable to find letter: ".red(),
                            letter_arg.to_lowercase().bold().red()
                        );
                    }
                }
            };
        }
        None => {
            println!("\n{}\n", "R you lost mate?".bold().bright_cyan());
            let mut cmd = Args::command();
            cmd.print_help().unwrap();
        }
    }

    Ok(())
}

const GREEK_ALPHABET_JSON: &str = r#"
{
    "letters": [
      {
        "name" : "alpha",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "beta",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "gamma",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "delta",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "epsilon",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "zeta",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "eta",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "theta",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "iota",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "kappa",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "lambda",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "mu",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "nu",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "xi",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "omicron",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "pi",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "rho",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "sigma",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "tau",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "upsilon",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "phi",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "chi",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "psi",
        "lower": "??",
        "upper": "??"
      },
      {
        "name" : "omega",
        "lower": "??",
        "upper": "??"
      }
    ]
  }
  "#;

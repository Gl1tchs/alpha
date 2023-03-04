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
        "lower": "α",
        "upper": "Α"
      },
      {
        "name" : "beta",
        "lower": "β",
        "upper": "Β"
      },
      {
        "name" : "gamma",
        "lower": "γ",
        "upper": "Γ"
      },
      {
        "name" : "delta",
        "lower": "δ",
        "upper": "Δ"
      },
      {
        "name" : "epsilon",
        "lower": "ε",
        "upper": "Ε"
      },
      {
        "name" : "zeta",
        "lower": "ζ",
        "upper": "Ζ"
      },
      {
        "name" : "eta",
        "lower": "η",
        "upper": "Η"
      },
      {
        "name" : "theta",
        "lower": "θ",
        "upper": "Θ"
      },
      {
        "name" : "iota",
        "lower": "ι",
        "upper": "Ι"
      },
      {
        "name" : "kappa",
        "lower": "κ",
        "upper": "Κ"
      },
      {
        "name" : "lambda",
        "lower": "λ",
        "upper": "Λ"
      },
      {
        "name" : "mu",
        "lower": "μ",
        "upper": "Μ"
      },
      {
        "name" : "nu",
        "lower": "ν",
        "upper": "Ν"
      },
      {
        "name" : "xi",
        "lower": "ξ",
        "upper": "Ξ"
      },
      {
        "name" : "omicron",
        "lower": "ο",
        "upper": "Ο"
      },
      {
        "name" : "pi",
        "lower": "π",
        "upper": "Π"
      },
      {
        "name" : "rho",
        "lower": "ρ",
        "upper": "Ρ"
      },
      {
        "name" : "sigma",
        "lower": "σ",
        "upper": "Σ"
      },
      {
        "name" : "tau",
        "lower": "τ",
        "upper": "Τ"
      },
      {
        "name" : "upsilon",
        "lower": "υ",
        "upper": "Υ"
      },
      {
        "name" : "phi",
        "lower": "φ",
        "upper": "Φ"
      },
      {
        "name" : "chi",
        "lower": "χ",
        "upper": "Χ"
      },
      {
        "name" : "psi",
        "lower": "ψ",
        "upper": "Ψ"
      },
      {
        "name" : "omega",
        "lower": "ω",
        "upper": "Ω"
      }
    ]
  }
  "#;

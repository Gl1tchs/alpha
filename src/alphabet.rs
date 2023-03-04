use colored::Colorize;
use prettytable::{Cell, Row, Table};
use serde::{Deserialize, Serialize};

use crate::capitalize_first;

#[derive(Serialize, Deserialize, Debug)]
pub struct Letter {
    pub name: String,
    pub lower: String,
    pub upper: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Alphabet {
    pub letters: Vec<Letter>,
}

impl Alphabet {
    pub fn import(content: &str) -> std::io::Result<Self> {
        let _: serde_json::Result<Alphabet> = match serde_json::from_str(&content) {
            Ok(alphabet) => return Ok(alphabet),
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                ));
            }
        };
    }

    pub fn print_table(self) {
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Name"),
            Cell::new("Lower"),
            Cell::new("Upper"),
        ]));
        for letter in self.letters {
            table.add_row(Row::new(vec![
                Cell::new(
                    capitalize_first(letter.name.as_str())
                        .green()
                        .bold()
                        .to_string()
                        .as_str(),
                ),
                Cell::new(letter.lower.bold().to_string().as_str()),
                Cell::new(letter.upper.bold().to_string().as_str()),
            ]));
        }
        table.printstd();
    }
}

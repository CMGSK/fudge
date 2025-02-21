use std::{env, process::Command};

use crate::{algorithms::metrics::find_by_custom_levenshtein, components::config::Configuration};

#[derive(Clone)]
pub struct Interceptor {
    command: Option<String>,
    correction: Option<String>,
}

impl Interceptor {
    pub fn new() -> Self {
        Interceptor {
            command: None,
            correction: None,
        }
    }

    pub fn get_last_command(&mut self) -> String {
        let shell = env::var("SHELL").unwrap_or(String::from("sh"));
        let out = Command::new(shell)
            .arg("-c")
            .arg("history | tail -n 1")
            .output()
            .expect("Fudge could not access terminal history");

        match out.status.success() {
            true => {
                let c = String::from_utf8_lossy(&out.stdout)
                    .split_once(' ')
                    .map(|(_, x)| x.to_string())
                    .expect("Fudge has encounter an error parsing the last command");

                self.command = Some(c.clone());
                return c;
            }
            false => String::from("Fudge!!"),
        }
    }

    pub fn correct(cfg: Configuration, command: String) -> String {
        match cfg.string_metric.as_str() {
            "levenshtein" => find_by_custom_levenshtein(cfg, command),
            _ => find_by_custom_levenshtein(cfg, command),
        }
    }
}

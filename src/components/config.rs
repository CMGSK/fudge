use std::{fs::File, io::BufReader, process::Command};

use serde::{Deserialize, Serialize};

const FETCH_COMMANDS_FUNC: &str = 
    r#"COMMANDS=`echo -n $PATH | xargs -d :
    -I {} find {} -maxdepth 1 \
    -executable -type f -printf '%P\n'`
    ALIASES=`alias | cut -d '=' -f 1`
    echo "$COMMANDS"$'\n'"$ALIASES" | sort -u"#;



    // "COMMANDS=`echo -n $PATH | xargs -d : -I {} find {} -maxdepth 1 \\ -executable -type f -printf '%P\\n'` 
    // ALIASES=`alias | cut -d '=' -f 1` 
    // echo \"$COMMANDS\"$'\\n'\"$ALIASES\" | sort -u";

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub shell: String,
    pub trigger: String,
    pub string_metric: String,
    pub haystack: Vec<String>,
    pub no_question: bool,
    pub custom_rules: Vec<String>,
    pub forbidden_commands: Vec<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            shell: String::from("bash"),
            trigger: "fudge".into(),
            string_metric: "custom_levenshtein".into(),
            haystack: get_term_command_list(),
            no_question: false,
            custom_rules: Vec::new(),
            forbidden_commands: vec!["sudo".into(), "su".into()],
        }
    }
}

impl Configuration {
    pub fn new() -> Self {
        let cfg_file = match File::open("~/.config/fudge/fudge.conf") {
            Ok(f) => Some(f),
            Err(_) => None,
        };
        let Some(cfg_file) = cfg_file else {
            return Configuration::default();
        };

        let reader = BufReader::new(cfg_file);
        let cfg: Configuration = serde_json::from_reader(reader).unwrap();

        cfg
    }

    pub fn update_commands(&mut self) {
        self.haystack = get_term_command_list();
    }

    pub fn update_custom_rules(&mut self) {
        let cfg_file = match File::open("~/.config/fudge/fudge.conf") {
            Ok(f) => f,
            Err(_) => {
                self.custom_rules = Vec::new();
                return;
            }
        };
        let reader = BufReader::new(cfg_file);
        let cfg: Configuration = serde_json::from_reader(reader).unwrap();
        self.custom_rules = cfg.custom_rules;
    }

    pub fn update_trigger(&mut self, new_trigger: String) {
        self.trigger = new_trigger;
    }

    pub fn update_metrics(&mut self, new_metric: String) {
        self.string_metric = new_metric;
    }

    pub fn update_question(&mut self) {
        self.no_question = !self.no_question;
    }

    pub fn update_forbidden_commands(&mut self, command: String, op: &str) {
        match op {
            "add" => self.forbidden_commands.push(command),
            "del" => {
                if let Some(idx) = self.forbidden_commands.iter().position(|x| *x == command) {
                    self.forbidden_commands.remove(idx);
                }
            }
            _ => (),
        };
    }
}

fn get_term_command_list() -> Vec<String> {
    if let Ok(output) = Command::new("zsh").arg("-c").arg(FETCH_COMMANDS_FUNC).output() {
        if let Ok(s) = String::from_utf8(output.stdout) {
            return s
                .split('\n')
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
        } else {
            panic!("Invalid stdout fetching terminal commands");
        }
    } else {
        panic!("Invalid terminal commands fetch function");
    }
}

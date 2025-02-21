mod algorithms;
mod components;

use components::{config::Configuration, interceptor::Interceptor};
use notify::{Config, RecommendedWatcher, Watcher};
use std::{fs::File, io::{self, BufRead, BufReader}, path::{Path, PathBuf}, sync::mpsc::channel, time::Duration};

fn main() -> io::Result<()> {
    let mut cfg = Configuration::default();

    // Channel to receive filesystem events
    let (tx, rx) = channel();
    let watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    let history_path = if std::path::Path::new("$HOME/.bash_history").is_file() {
        "$HOME/.bash_history"
    } else {
        "$HOME/.zsh_history"
    };
    watcher.watch(history_path.as_ref(), notify::RecursiveMode::NonRecursive);
    watcher.watch(
        "$HOME/.config/fudge".as_ref(),
        notify::RecursiveMode::NonRecursive,
    );

    let mut last_pos = get_file_size(&history_path)?;

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => {
                let Ok(event) = event else {
                    continue;
                };

                if event.paths.iter().any(|x| x.to_str().unwrap() == "$HOME/.config/fudge") {
                    if let Ok(new_cfg) = Ok(cfg) { //TODO load new config here
                        cfg = new_cfg;
                    } 
                }

            },
            Err(e) => todo!()
        }
    }

    Ok(())
}

fn get_history_file() -> io::Result<PathBuf> {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| String::from("/usr/bin/sh"));
    match shell.as_str() {
        "/usr/bin/zsh" => Ok(PathBuf::from("$HOME/.zsh_history")),
        "/usr/bin/bash" => Ok(PathBuf::from("$HOME/.bash_history")),
        _ => Ok(PathBuf::from("$HOME/.shell_history")),
    }
}

fn get_file_size(path: &str) -> io::Result<u64> {
    let path = Path::new(path);
    Ok(path.metadata()?.len())
}

fn get_last_command(path: &Path, leof: &mut u64) -> Result<String, ()> {
    let file = File::open(path).unwrap();
    let size = file.metadata().unwrap().len();
    
    if size > *leof {
        let f = BufReader::new(file);
        if let Some(nc) = f.lines().last() {
            match nc {
                Ok(nc) => {
                    *leof = size;
                    return Ok(nc);
                }
                Err(_) => return Err(()),
            }
        } 
        return Err(());
    }
    Err(())
}

fn display(command: &str) {
    print!("[Fudge] Did you mean: {} ? Y/n", command);
    let mut reply = String::new();
    io::stdin().read_line(&mut reply).unwrap();
    if reply.is_empty() || reply.to_lowercase() == "y" {
        fudge(command);
    } 
}

fn fudge(command: &str) {
        Command::new(shell)
            .arg("-c")
            .arg("history | tail -n 1")
            .output()
            .expect("Fudge could not access terminal history");
}
extern crate serde;

mod ui;

use std::convert::TryFrom;
use std::sync::mpsc;
use ui::action::Action;

pub fn run_loop() {
    let (tx_ui_update, rx_ui_update) = mpsc::channel::<Action>();

    let mut current_state = ui::state::State::new();
    output_state(&current_state);

    let _in_thread = std::thread::spawn(move || loop {
        let mut command = String::new();
        match std::io::stdin().read_line(&mut command) {
            Ok(_in_bytes) => {
                parse_command(command).map(|action| {
                    tx_ui_update.send(action).expect("TODO: Handle tx error.");
                });
            }
            Err(_err) => break,
        }
    });

    print_help();

    for command in rx_ui_update {
        match ui::action::Action::try_from(command) {
            Ok(action) => {
                let next_state = current_state.reduce_command(&action);
                // TODO: Replace this with a diff.
                output_state(&next_state);
                current_state = next_state;
            }
            Err(err) => {
                println!("Invalid command: '{}'", err);
            }
        }

        if current_state.is_done() {
            println!("We are done.");
            break;
        }
    }

    println!("Quit requested. Waiting for jobs to complete...");
    // TODO: tell input sources to close and wait for them.
    println!("...done. Goodbye!  And thank you for using Procol!");
}

fn output_state(state: &ui::state::State) {
    println!("state: {}", state.to_json());
}

fn parse_command(command: String) -> Option<Action> {
    let trimmed = command.trim();
    if trimmed == "/quit" {
        return Some(Action::Quit);
    }
    if trimmed.starts_with("/send ") {
        let input = trimmed[6..].to_string();
        return Some(Action::Send { input });
    }

    return Some(Action::Send {
        input: trimmed.to_string(),
    });
}

fn print_help() {
    println!("To Quit: {}", serde_json::to_string(&Action::Quit).unwrap());
    println!(
        "To Send: {}",
        serde_json::to_string(&Action::Send {
            input: "message".to_string()
        })
        .unwrap()
    );
}

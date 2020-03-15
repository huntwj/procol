extern crate procol;

use procol::ui_state;
use std::sync::mpsc;

fn main() {
    let (tx_ui_update, rx_ui_update) = mpsc::channel::<String>();
    let out_thread = std::thread::spawn(move || {
        let mut current_state = ui_state::UiState::new();
        output_state(&current_state);
        for command in rx_ui_update {
            let next_state = current_state.reduce_command(&command);
            // TODO: Replace this with a diff.
            output_state(&next_state);
            current_state = next_state;

            if current_state.is_done() {
                break;
            }
        }
    });

    loop {
        let mut command = String::new();
        match std::io::stdin().read_line(&mut command) {
            Ok(_in_bytes) => {
                let trimmed = command.trim();
                tx_ui_update
                    .send(trimmed.to_string())
                    .expect("TODO: Handle tx error.");
                if trimmed == "quit" {
                    break;
                }
            }
            Err(_err) => break,
        }
    }

    println!("Quit requested. Waiting for jobs to complete...");
    // Wait for all work to finishing.
    out_thread.join().unwrap();
    println!("...done. Goodbye!  And thank you for using Procol!");
}

fn output_state(state: &ui_state::UiState) {
    println!("{{ \"_t:\": \"state\", \"done\": {} }}", state.is_done());
}

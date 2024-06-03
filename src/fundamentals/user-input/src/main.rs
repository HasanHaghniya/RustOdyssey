use std::io::stdin;

enum PowerActions {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerActions {
    fn new(action: &str) -> Option<PowerActions> {
        let action = action.trim().to_lowercase();

        match action.as_str() {
            "off" => Some(PowerActions::Off),
            "sleep" => Some(PowerActions::Sleep),
            "reboot" => Some(PowerActions::Reboot),
            "shutdown" => Some(PowerActions::Shutdown),
            "hibernate" => Some(PowerActions::Hibernate),
            _ => None,
        }
    }
}

fn handle_user_input(input: PowerActions) {
    use PowerActions::*;

    match input {
        Off => println!("turning off"),
        Sleep => println!("going to sleep"),
        Reboot => println!("rebooting"),
        Shutdown => println!("shutting down"),
        Hibernate => println!("hibernating"),
    }
}

fn main() {
    let mut buffer = String::new();

    let user_input = stdin().read_line(&mut buffer);

    if user_input.is_ok() {
        match PowerActions::new(&buffer) {
            Some(action) => handle_user_input(action),
            None => println!("invalid power action"),
        }
    }
}

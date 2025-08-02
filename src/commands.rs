use super::MatchState;

pub enum Cmd {
    Route,
    Build,
    Expand,
    Bits,
    Unknown,
}

pub fn issue_command(ms: &mut MatchState, command: Cmd, args: Vec<String>) -> String {
    match command {
        Cmd::Bits => {
            if args.len() >= 1 {
                if let Ok(n_bits) = args[0].parse::<i32>() {
                    ms.player.bits += n_bits;
                    format!(">> You got {} bits!", args[0])
                } else {
                    format!("Invalid argument!")
                }
            } else {
                format!("What?!")
            }
        }
        _ => format!("Nope!"),
    }
}

pub fn parse_commands(input: String) -> (Cmd, Vec<String>) {
    let split_input: Vec<&str> = input.split_whitespace().collect();
    let command = split_input[0];

    let args: Vec<&str>;
    let mut output = vec![];

    if split_input.len() > 1 {
        args = split_input[1..].to_vec();
        for arg in args {
            output.push(arg.to_owned());
        }
    }

    match command {
        "route" => return (Cmd::Route, output),
        "bits" => return (Cmd::Bits, output),
        _ => return (Cmd::Unknown, output),
    }
}

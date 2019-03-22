use std::io;
use std::io::Write;
use std::io::stdout;
use std::process::{exit, Command, Stdio};

#[derive(Debug, Clone)]
struct Cmd {
    command: String,
    args: Vec<String>
}

fn spawn_command(cmd: &Cmd) -> bool {
    let flag = cmd.args.last();
    if flag != None && flag.unwrap() == "&" {
        match Command::new(cmd.command.clone())
            .args(cmd.args.clone())
            .stdout(Stdio::piped())
            .spawn() {
                Ok(_) => {
                    true
                },
                Err(_) => {
                    eprintln!("Unknown command: {}", cmd.command);
                    false
                }
            }
    } else {
        match Command::new(cmd.command.clone())
            .args(cmd.args.clone())
            .spawn() {
                Ok(mut child) => {
                    child.wait().unwrap();
                    true
                },
                Err(_) => {
                    eprintln!("Unknown command: {}", cmd.command);
                    false
                }
            }
        
    }
}

fn format_args(cmd: &Cmd) -> String {
    let mut args = cmd.args.iter()
        .fold(String::new(),
              |acc, arg| acc + arg + " ");
    args.pop();
    args
}

fn launch(cmd: &Cmd, stack: &mut Vec<Cmd>) {
    match cmd.command.as_str() {
        "exit" => exit(0),
        "!!" => {
            match stack.len() {
                0 => {
                    eprintln!("No history command.");
                },
                _ => {
                    let prev = Cmd {
                        command: stack.last().unwrap().command.clone(),
                        args: stack.last().unwrap().args.clone()
                    };
                    println!("{} {}", prev.command, format_args(&prev));
                    launch(&prev, stack);
                }
            }
        },
        "history" => {
            stack.push(Cmd {
                command: "history".into(),
                args: vec![]
            });
            for (i, c) in stack.iter().rev().enumerate() {
                println!("{} {} {}",
                         stack.len() - i,
                         c.command,
                         format_args(&c));
            }
        },
        _ => {
            let text = cmd.command.chars();
            if text.clone().next().unwrap() == '!' {
                let st = &cmd.command[1..];
                match st.parse::<usize>() {
                    Ok(n) => {
                        if n > stack.len() {
                            eprintln!("Index out of bound");
                        } else {
                            let cmd = stack[n - 1].clone();
                            println!("{} {}",
                                     cmd.command,
                                     format_args(&cmd));
                            launch(&cmd, stack);
                        }
                    },
                    Err(_) => { eprintln!("Wrong command");}
                };
            } else {
                match spawn_command(&cmd) {
                    true => {stack.push(cmd.clone())},
                    false => {}
                }
            }
        }
    }
}

fn main() {
    let mut stack: Vec<Cmd> = vec![];
    loop {
        print!("rush> ");
        stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let mut parts = input.split_whitespace();
        let built = Cmd {
            command: parts.next().unwrap().into(),
            args: parts.map(|s| s.into()).collect()
        };
        launch(&built, &mut stack);
    }
}

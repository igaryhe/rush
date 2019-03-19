use std::io;
use std::io::Write;
use std::io::stdout;
use std::process::{exit, Command, Stdio};

#[derive(Debug, Clone)]
struct Cmd {
    command: String,
    args: Vec<String>
}

fn spawn_command(cmd: Cmd, stack: &mut Vec<Cmd>) {
    if cmd.command == "history" {
        history(stack);
    } else {
        let flag = cmd.args.last().clone();
        if flag != None && flag.unwrap() == "&" {
            match Command::new(cmd.command.clone())
                .args(cmd.args.clone())
                .stdout(Stdio::piped())
                .spawn() {
                    Ok(_) => {},
                    Err(_) => eprintln!("Unknown command: {}", cmd.command.clone())
                };
        } else {
            match Command::new(cmd.command.clone())
                .args(cmd.args.clone())
                .spawn() {
                    Ok(mut child) => {
                        child.wait().unwrap();
                    },
                    Err(_) => eprintln!("Unknown command: {}", cmd.command.clone())
                };
        }
    }
}

fn history(stack: &mut Vec<Cmd>) {
    for (i, c) in stack.iter().rev().enumerate() {
        println!("{} {} {}",stack.len() - i , c.command, c.args.iter()
                 .fold(String::new(), |acc, arg| acc + arg + " "));
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
            command: parts.next().unwrap().to_string(),
            args: parts.map(|s| s.to_string()).collect()
        };
        match built.command.as_str() {
            "exit" => exit(0),
            
            "!!" => {
                if stack.len() == 0 {
                    eprintln!("No command in history");
                } else {
                    let prev = Cmd {
                        command: stack.last().unwrap().command.clone(),
                        args: stack.last().unwrap().args.clone()
                    };
                    stack.push(prev.clone());
                    println!("{} {}", prev.command.clone(), prev.args.iter()
                             .fold(String::new(), |acc, arg| acc + arg + " "));
                    spawn_command(prev, &mut stack);
                }
            }
            _ => {
                let text = built.command.chars();
                if text.clone().next().unwrap() == '!' {
                    let st = &built.command[1..];
                    match st.parse::<usize>() {
                        Ok(n) => {
                            if n > stack.len() - 1 {
                                eprintln!("Index out of bound");
                            } else {
                                let cmd = stack[n - 1].clone();
                                stack.push(cmd.clone());
                                println!("{} {}", cmd.command.clone(), cmd.args.iter()
                                         .fold(String::new(), |acc, arg| acc + arg + " "));
                                spawn_command(cmd, &mut stack);
                            }
                        },
                        Err(_) => { eprintln!("Wrong command");}
                    };
                } else {
                    stack.push(built.clone());
                    spawn_command(built, &mut stack)
                }
            }
        }
    }
}

use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::Stdio;
use std::process::Child;
use std::process::Command;
use std::path::Path;
use std::env;
use colored::Colorize;

fn main(){
    let rustex_logo = r"______ _   _ _____ _____ _______   __
| ___ \ | | /  ___|_   _|  ___\ \ / /
| |_/ / | | \ `--.  | | | |__  \ V / 
|    /| | | |`--. \ | | |  __| /   \ 
| |\ \| |_| /\__/ / | | | |___/ /^\ \
\_| \_|\___/\____/  \_/ \____/\/   \/";
    println!("{}", format!("{}",rustex_logo).purple());
    loop {
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // must be peekable so we know when we are on the last command
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next()  {

            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek()
                        .map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                },
                "exit" => return,
                command => {
                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            final_command.wait();
        }

    }
}
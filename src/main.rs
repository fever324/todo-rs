use std::fmt::Display;

use clap::Parser;
use serde::{Deserialize, Serialize};

type Todos = Vec<Item>;

const FILE_NAME: &str = "todo.json";

#[derive(Debug, Copy, Clone)]
enum Command {
    Add,
    Print,
    Exit,
    Check,
    Remove,
    Continue,
}

#[derive(Parser, Debug, Clone)]
struct Cli {
    command: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    completed: bool,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.completed {
            return write!(f, "[x] {}", &self.name);
        }

        write!(f, "[ ] {}", &self.name)
    }
}

const USER_COMMANDS: [Command; 5] = [
    Command::Add,
    Command::Check,
    Command::Remove,
    Command::Print,
    Command::Exit,
];

fn main() {
    clear_screen();
    let args = Cli::parse();
    let command_str = args.command.as_deref();

    let mut command = get_command(command_str);
    loop {
        let mut todos = read_from_file();
        process_command(command, &mut todos);
        write_to_file(&todos).unwrap();
        command = get_new_command();
    }
}

fn get_new_command() -> Command {
    println!("Enter command: ");
    let strs: Vec<String> = USER_COMMANDS
        .iter()
        .map(|c| " - ".to_owned() + &get_command_string(*c) + "\n")
        .collect();
    println!("OPTIONS: \n{}", strs.join(""));

    println!("\n");
    let input = get_user_input();

    get_command(Some(&input))
}

fn get_user_input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn get_command(command_str: Option<&str>) -> Command {
    match command_str {
        Some("add" | "a") => Command::Add,
        Some("check" | "c" | "uncheck" | "u") => Command::Check,
        Some("remove" | "r") => Command::Remove,
        Some("print" | "p") => Command::Print,
        Some("exit" | "e") => Command::Exit,
        None => Command::Continue,
        _ => {
            println!("No Command called {}", command_str.unwrap());
            Command::Continue
        }
    }
}

fn get_command_string(command: Command) -> String {
    match command {
        Command::Add => "(a)dd".to_string(),
        Command::Print => "(p)rint".to_string(),
        Command::Exit => "(e)xit".to_string(),
        Command::Check => "(c)heck/uncheck".to_string(),
        Command::Remove => "(r)emove".to_string(),
        Command::Continue => panic!("Should not happen"),
    }
}

fn process_command(command: Command, todos: &mut Todos) {
    clear_screen();
    match command {
        Command::Add => {
            add_todo(todos);
            print_todo(todos, false);
        }
        Command::Check => {
            check_todo(todos);
            print_todo(todos, false);
        }
        Command::Print => {
            print_todo(todos, false);
        }
        Command::Exit => {
            std::process::exit(1);
        }
        Command::Remove => {
            remove_todo(todos);
            print_todo(todos, false);
        }
        Command::Continue => {}
    }
}

fn write_to_file(todos: &Todos) -> std::io::Result<()> {
    let serialized = serde_json::to_string(todos).unwrap();
    std::fs::write(FILE_NAME, &serialized)
}

fn read_from_file() -> Todos {
    let content = std::fs::read_to_string(FILE_NAME).unwrap_or_default();

    serde_json::from_str(&content).unwrap_or_default()
}

fn add_todo(todos: &mut Todos) {
    println!("What's the Todo's name?");
    let line = get_user_input();
    println!("\n");
    let item = Item {
        name: line,
        completed: false,
    };

    todos.push(item);
    clear_screen();
}

fn check_todo(todos: &mut Todos) {
    if todos.is_empty() {
        return;
    }

    let index = get_operation_index(todos);
    todos[index].completed = !todos[index].completed;
    clear_screen();
}

fn remove_todo(todos: &mut Todos) {
    if todos.is_empty() {
        return;
    }

    let index = get_operation_index(todos);
    todos.remove(index);
    clear_screen();
}

fn get_operation_index(todos: &Todos) -> usize {
    println!("Which one?");
    print_todo(todos, true);

    let mut line = get_user_input();

    let mut index = line.parse::<usize>().ok();
    while index.is_none() || index >= Some(todos.len()) {
        println!("\nInvalid input. Try again");
        line = get_user_input();

        index = line.parse::<usize>().ok();
    }

    println!("\n");
    index.unwrap()
}

fn print_todo(todos: &Todos, show_index: bool) {
    if todos.is_empty() {
        println!("[Empty Todo List]");
    }

    for (i, item) in todos.iter().enumerate() {
        let index_str = if show_index {
            i.to_string() + " "
        } else {
            String::new()
        };

        println!("{}{}", index_str, item);
    }
    println!();
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

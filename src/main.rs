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

impl Item {
    pub fn to_string(&self) -> String {
        if self.completed {
            return format!("[x] {}", &self.name);
        }
        return format!("[ ] {}", &self.name);
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
        write_to_file(todos).unwrap();
        command = get_new_command();
    }
}

fn get_new_command() -> Command {
    println!("Enter command: ");
    let strs: Vec<String> = USER_COMMANDS
        .iter()
        .map(|c| " - ".to_owned() + &get_command_string(*c).to_string() + "\n")
        .collect();
    println!("OPTIONS: \n{}", strs.join(""));

    println!("\n");
    let input = get_user_input();

    return get_command(Some(&input));
}

fn get_user_input()-> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn get_command(command_str: Option<&str>) -> Command {
     match command_str.as_deref() {
        Some("add") | Some("a") => Command::Add,
        Some("check") | Some("c") => Command::Check,
        Some("uncheck") | Some("u") => Command::Check,
        Some("remove") | Some("r") => Command::Remove,
        Some("print") | Some("p") => Command::Print,
        Some("exit") | Some("e") => Command::Exit,
        None => Command::Continue,
        _ => {
            println!("No Command called {}", command_str.unwrap());
            return Command::Continue;
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
        _ => panic!("Should not happen"),
    }
}

fn process_command(command: Command, todos: &mut Todos) -> () {
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

fn write_to_file(todos: Todos) -> std::io::Result<()> {
    let serialized = serde_json::to_string(&todos).unwrap();
    return std::fs::write(FILE_NAME, &serialized);
}

fn read_from_file() -> Todos {
    let content = std::fs::read_to_string(FILE_NAME).unwrap();

    let deserialized: Todos = serde_json::from_str(&content).unwrap();
    return deserialized;
}

fn add_todo(todos: &mut Todos) {
    println!("What's the Todo's name?");
    let line = get_user_input();
    println!("\n");
    let item = Item {
        name: line.to_owned(),
        completed: false,
    };

    todos.push(item);
    clear_screen();
}

fn check_todo(todos: &mut Todos) {
    let index = get_operation_index(todos);
    todos[index as usize].completed = !todos[index as usize].completed;
    clear_screen();
}

fn remove_todo(todos: &mut Todos) {
    let index = get_operation_index(todos);
    todos.remove(index as usize);
    clear_screen();
}

fn get_operation_index(todos: &Todos) -> u32 {
    println!("Which one?");
    print_todo(todos, true);

    let line = get_user_input();
    println!("\n");

    return line.parse::<u32>().unwrap();
}

fn print_todo(todos: &Todos, show_index: bool) {
    for (i, item) in todos.iter().enumerate() {
        let index_str = if show_index {
            i.to_string() + " "
        } else {
            "".to_owned()
        };

        println!("{}{}", index_str, item.to_string());
    }
    println!();
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

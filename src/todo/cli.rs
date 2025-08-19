use clap::{arg, Command};
pub enum TodoCommand {
    Add(String),
    Toggle(usize),
    Check(usize),
    Uncheck(usize),
    Remove(usize),
    List,
    None,
}

impl TodoCommand {
    pub fn parse_command() -> Self {
        let matches = Command::new("todo")
        .about("A simple CLI Todo application")
        .subcommand(Command::new("add").arg(arg!(<TASK>)))
        .subcommand(Command::new("toggle").arg(arg!(<INDEX>)))
        .subcommand(Command::new("check").arg(arg!(<INDEX>)))
        .subcommand(Command::new("uncheck").arg(arg!(<INDEX>)))
        .subcommand(Command::new("remove").arg(arg!(<INDEX>)))
        .subcommand(Command::new("list"))
        .get_matches();

        
        if let Some(m) = matches.subcommand_matches("add") {
            if let Some(task) = m.get_one::<String>("TASK") {
                return TodoCommand::Add(task.clone());
            }
        }
        if let Some(m) = matches.subcommand_matches("toggle") {
            if let Some(idx) = m.get_one::<String>("INDEX") {
                return TodoCommand::Toggle(idx.parse().expect("Invalid index"));
            }
        }
        if let Some(m) = matches.subcommand_matches("check") {
            if let Some(idx) = m.get_one::<String>("INDEX") {
                return TodoCommand::Check(idx.parse().expect("Invalid index"));
            }
        }
        if let Some(m) = matches.subcommand_matches("uncheck") {
            if let Some(idx) = m.get_one::<String>("INDEX") {
                return TodoCommand::Uncheck(idx.parse().expect("Invalid index"));
            }
        }
        if let Some(m) = matches.subcommand_matches("remove") {
            if let Some(idx) = m.get_one::<String>("INDEX") {
                return TodoCommand::Remove(idx.parse().expect("Invalid index"));
            }
        }
        if matches.subcommand_matches("list").is_some() {
            return TodoCommand::List;
        }

        TodoCommand::None
    }
}
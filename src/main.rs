mod todo;

use todo::{Ledger, TodoCommand};

fn main() {
    let command = TodoCommand::parse_command();
    let mut ledger = Ledger::new();

    match command {
        TodoCommand::Add(task)    => ledger.add(&task),
        TodoCommand::Toggle(idx)  => ledger.toggle(idx),
        TodoCommand::Check(idx)   => ledger.mark(idx, true),
        TodoCommand::Uncheck(idx) => ledger.mark(idx, false),
        TodoCommand::Remove(idx)  => ledger.remove(idx),
        TodoCommand::List         => ledger.list(),
        TodoCommand::None         => {
            println!("No command provided. Run `todo --help` for usage.");
        }
    }
}

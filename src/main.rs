extern crate structopt;
use structopt::StructOpt;

pub mod db;
pub mod commands;

use commands::CommandAction;


mod strategies;
use strategies::{CreateStrategy, RemoveStrategy, StartStrategy, StopStrategy, ListStrategy};

fn main() {
    let opt = CommandAction::from_args();
    println!("{:?}", opt);

    match opt {
        CommandAction::Create { inactive, name } => CreateStrategy::handle(inactive, name),
        CommandAction::Remove { id } => RemoveStrategy::handle(id),
        CommandAction::Start { id } => StartStrategy::handle(id),
        CommandAction::Stop { id } => StopStrategy::handle(id),
        CommandAction::List { active, inactive, name } => ListStrategy::handle(active, inactive, name),
    }
}

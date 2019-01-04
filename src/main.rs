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
        CommandAction::Create(create_opts) => CreateStrategy::handle(create_opts),
        CommandAction::Remove(single_id) => RemoveStrategy::handle(single_id),
        CommandAction::Start(single_id) => StartStrategy::handle(single_id),
        CommandAction::Stop(single_id) => StopStrategy::handle(single_id),
        CommandAction::List(list_opts) => ListStrategy::handle(list_opts),
    }
}

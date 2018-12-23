extern crate structopt;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum CommandAction {
    #[structopt(name = "create")]
    Create {
        #[structopt(short = "i")]
        inactive: bool,
        name: String,
    },
    #[structopt(name = "remove")]
    Remove {
        id: i32
    },
    #[structopt(name = "start")]
    Start {
        id: i32
    },
    #[structopt(name = "stop")]
    Stop {
        id: i32
    },
    #[structopt(name = "list")]
    List {
        #[structopt(short = "a")]
        active: bool,
        #[structopt(short = "i")]
        inactive: bool,
        name: Option<String>
    },
}

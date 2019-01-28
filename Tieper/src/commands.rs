extern crate structopt;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum CommandAction {
    #[structopt(name = "create")]
    Create(CreateOpts),
    #[structopt(name = "remove")]
    Remove(SingleId),
    #[structopt(name = "start")]
    Start(SingleId),
    #[structopt(name = "stop")]
    Stop(SingleId),
    #[structopt(name = "list")]
    List(ListOpts),
}

#[derive(StructOpt, Debug)]
pub struct SingleId {
    pub id: i32
}

#[derive(StructOpt, Debug)]
pub struct CreateOpts {
    #[structopt(short = "i")]
    pub inactive: bool,
    pub name: String
}

#[derive(StructOpt, Debug)]
pub struct ListOpts {
    #[structopt(short = "a")]
    pub active: bool,
    #[structopt(short = "i")]
    pub inactive: bool,
    pub name: Option<String>
}
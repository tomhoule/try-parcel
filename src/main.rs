extern crate dotenv;
extern crate yacchauyo;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "server")]
enum Command {
    #[structopt(name = "rpc")]
    Rpc,
    #[structopt(name = "web")]
    Web,
}

fn main() {
    dotenv::dotenv().ok();
    match Command::from_args() {
        Command::Rpc => {
            yacchauyo::start_rpc();
        },
        Command::Web => {
            yacchauyo::start_web();
        }
    }
}

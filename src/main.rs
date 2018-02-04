extern crate dotenv;
extern crate yacchauyo;

fn main() {
    dotenv::dotenv().ok();
    // yacchauyo::start_rpc();
    yacchauyo::start_web();
}

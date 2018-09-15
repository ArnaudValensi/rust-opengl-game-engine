extern crate bigseed;
extern crate pretty_env_logger;
extern crate dotenv;

use bigseed::game;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    game();
}

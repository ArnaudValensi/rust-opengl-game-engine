use errors::*;
use specs::World;
use components::transform::Transform;

fn run() -> Result<()> {
    println!("Hi!");

    let mut world = World::new();

    world.register::<Transform>();

    Ok(())
}

fn print_errors_and_exit(e: &Error) {
    println!("error: {}", e);

    for e in e.iter().skip(1) {
        println!("caused by: {}", e);
    }

    // The backtrace is not always generated. Try to run this example
    // with `RUST_BACKTRACE=1`.
    if let Some(backtrace) = e.backtrace() {
        println!("backtrace: {:?}", backtrace);
    }

    ::std::process::exit(1);
}

pub fn main_100_1() {
    if let Err(ref e) = run() {
        print_errors_and_exit(e);
    }
}

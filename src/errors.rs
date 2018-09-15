use failure::Error;

// TODO: Implement this style:
// ```
// error: could not download file from 'https://static.rust-lang.org/rustup/release-stable.toml' to '/tmp/rustup-update.T7Ry8gjuIytx/release-stable.toml'
// info: caused by: error during download
// info: caused by: [7] Couldn't connect to server (Failed to receive SOCKS4 connect request ack.)
// ```
pub fn print_errors_and_exit(error: &Error) {
    let mut causes = error.iter_chain();

    if let Some(cause) = causes.next() {
        println!("error: {}", cause);
    }

    for cause in causes {
        println!("caused by: {}", cause);
    }

    // `RUST_BACKTRACE=1` needs to be set to generate the backtrace.
    if let Some("1") = option_env!("RUST_BACKTRACE") {
        println!("{}", error.backtrace());
    }

    ::std::process::exit(1);
}

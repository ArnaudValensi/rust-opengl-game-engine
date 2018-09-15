use failure::Error;

pub fn print_errors_and_exit(error: &Error) {
    let mut causes = error.iter_chain();

    if let Some(cause) = causes.next() {
        error!("{}", cause);
    }

    for cause in causes {
        info!("caused by: {}", cause);
    }

    // `RUST_BACKTRACE=1` needs to be set to generate the backtrace.
    if let Some("1") = option_env!("RUST_BACKTRACE") {
        info!("{}", error.backtrace());
    }

    ::std::process::exit(1);
}

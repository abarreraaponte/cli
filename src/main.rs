fn main() {
    if let Err(err) = ctx::cli::run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

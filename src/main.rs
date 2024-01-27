fn main() {
    if let Err(e) = tasklite::get_args().and_then(tasklite::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

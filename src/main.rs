fn main() {
    if let Err(e) = todo::get_args().and_then(todo::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

mod cmd;

fn main() {
    if let Err(e) = cmd::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}

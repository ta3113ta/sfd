use structopt::StructOpt;

mod cmd;
mod opt;

fn main() {
    let opt = opt::Opt::from_args();
    if let Err(e) = cmd::run(opt) {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "sfd", about = "sfd")]
pub struct Opt {
    /// Deploy both dev and prod environments
    #[structopt(short, long)]
    pub all: bool,
}

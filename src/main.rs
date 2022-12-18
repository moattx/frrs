use clap::Parser;
use frrs::Cli;

fn main() {
    let mut cli = Cli::parse();
    //dbg!(&cli);
    cli.run();
}

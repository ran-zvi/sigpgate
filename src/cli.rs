use clap::{AppSettings, Clap};
use nix::sys::signal::Signal;


#[derive(Clap)]
#[clap(version = "1.0", author = "Ran Z. <rantzvi@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long)]
    pub pid: u32,
    
    #[clap(short, long)]
    pub verbose: bool,
    
    #[clap(subcommand)]
    pub mode: Subcommand

}

#[derive(Clap)]
pub enum Subcommand {
    Propagate(Propagate)
}

#[derive(Clap)]
pub struct Propagate {
    #[clap(short, long, default_value="SIGTERM")]
    pub signal: Signal,

    #[clap(short, long, default_value="0")]
    pub depth: u8,

    #[clap(short, long)]
    pub keep_alive: bool
}
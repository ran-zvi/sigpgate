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
    
    #[clap(short, long, default_value="SIGTERM")]
    pub listen_signal: Signal,

    #[clap(short, long)]
    pub send_signal: Option<Signal>,

    #[clap(short, long, default_value="0")]
    pub depth: u8,

    #[clap(short, long)]
    pub keep_alive: bool
}
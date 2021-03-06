use clap::{AppSettings, Clap};
use nix::sys::signal::Signal;

#[derive(Clap)]
#[clap(version = "1.0", author = "Ran Z. <rantzvi@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long)]
    pub cmd: Option<String>,

    #[clap(short, long)]
    pub pid: Option<u32>,

    #[clap(short, long)]
    pub verbose: bool,

    #[clap(short, long, default_value = "SIGTERM")]
    pub listen_signal: Signal,

    #[clap(short, long)]
    pub send_signal: Option<Signal>,

    #[clap(short, long, default_value = "1000")]
    pub wait_for_process_time: u64,

    #[clap(short, long, default_value = "0")]
    pub depth: u8,

    #[clap(short, long)]
    pub keep_alive: bool,

}

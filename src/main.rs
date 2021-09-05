mod cli;
mod errors;
mod listener;
mod process;
mod propagate;
mod traits;
mod types;

use crate::cli::Opts;
use crate::propagate::propagate_signal_to_all_child_pids;
use crate::types::SignalMap;

use clap::Clap;


fn main() {
    let opts: Opts = Opts::parse();
    match opts {
        Opts {
            pid,
            verbose,
            listen_signal,
            send_signal,
            depth,
            keep_alive,
        } => {
            let signal_map = SignalMap::new(listen_signal, send_signal);
            if keep_alive {
                loop {
                    match propagate_signal_to_all_child_pids(opts.pid as i32, depth, signal_map) {
                        Ok(_) => (),
                        Err(_) => {
                            if opts.verbose {
                                println!(
                                    "Pid at level {} of parent: {} not yet found, sleeping...",
                                    depth, opts.pid
                                )
                            }
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            } else {
                propagate_signal_to_all_child_pids(opts.pid as i32, depth, signal_map).unwrap();
            }
        }
    }
}

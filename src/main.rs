mod cli;
mod errors;
mod listener;
mod process;
mod propagate;
mod traits;
mod types;

use crate::cli::Opts;
use crate::errors::ProcessError;
use crate::propagate::propagate_signal_to_all_child_pids;
use crate::types::{Result, SignalMap};

use clap::Clap;
use std::process::Command;

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let pid = match opts.pid {
        Some(pid) => pid,
        None => match opts.cmd {
            Some(cmd) => spawn_child(cmd)?,
            None => panic!("Must provide either pid or cmd")
        }
    };
    

    let signal_map = SignalMap::new(opts.listen_signal, opts.send_signal);
    if opts.keep_alive {
        loop {
            match propagate_signal_to_all_child_pids(pid, opts.depth, signal_map) {
                Ok(_) => (),
                Err(_) => {
                    if opts.verbose {
                        println!(
                            "Pid at level {} of parent: {} not yet found, sleeping...",
                            opts.depth, pid
                        )
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    } else {
        propagate_signal_to_all_child_pids(pid, opts.depth, signal_map)
    }
}

fn spawn_child(command: String) -> Result<u32> {
    if command.len() == 0 {
        return Err(ProcessError::EmptyCommand.into());
    }
    let command: Vec<&str> = command.split_whitespace().collect();
    let mut cmd = Command::new(&command[0]);
    cmd.args(&command[1..]);
    let child = cmd.spawn()?;
    Ok(child.id())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_child() {
        spawn_child("true".into()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_spawn_child_invalid_command() {
        spawn_child("truf".into()).unwrap();
    }


    #[test]
    #[should_panic(expected="Cannot execute empty command")]
    fn test_spawn_child_empty_command() {
        spawn_child("".into()).unwrap();
    }
}
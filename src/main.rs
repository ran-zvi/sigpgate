mod cli;
mod errors;
mod listener;
mod process;
mod traits;
mod types;

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

use std::collections::HashMap;

use crate::cli::{Opts, Propagate, Subcommand};
use crate::errors::ProcessError;
use crate::listener::SignalListener;
use crate::process::{get_child_pids_full_tree};
use crate::traits::Listen;
use crate::types::{ChildPids, Result};

use clap::Clap;

fn get_child_pids_at_depth(pid: i32, depth: u8) -> Result<Option<ChildPids>> {
    let child_pids_by_depth: HashMap<_, _> = get_child_pids_full_tree(pid, None, None)?;
    let child_pids_at_depth = child_pids_by_depth.get(&(depth as usize));
    match child_pids_at_depth {
        Some(pids) => Ok(Some(pids.to_vec())),
        None => Ok(None),
    }
}

fn propagate_signal_to_all_child_pids(pid: i32, depth: u8, signal: Signal) -> Result<()> {
    let target_pids = get_target_pids(pid, depth)?;

    let mut join_handles: Vec<std::thread::JoinHandle<_>> = vec![];
    
    for pid in target_pids.into_iter() {
        let handle = std::thread::spawn(move || {
            propagate_signal(pid.clone(), signal)
        });
        join_handles.push(handle);
    }
    for handle in join_handles.into_iter() {
        handle.join().unwrap()?;
    }
    Ok(())
}

fn propagate_signal(pid: i32, signal: Signal) -> Result<()> {
    let listener = SignalListener::new(pid, signal);
    listener.listen()?;

    let child_pids = get_child_pids_full_tree(pid, None, None)?;
    for i in 1..=child_pids.len() as u8 {
        for pid in child_pids.get(&(i as usize)).unwrap().iter() {
            if let Ok(_) = kill(Pid::from_raw(*pid), signal) {
                println!("Killed child process: {} successfully", pid);
            }
            else {
                println!("Unable to kill child process: {}", pid);
            }
            
        }
    }
    Ok(())
}

fn get_target_pids(pid: i32, depth: u8) -> Result<ChildPids> {
    if depth == 0 {
        return Ok(vec![pid]);
    }

    if let Some(children) = get_child_pids_at_depth(pid, depth)? {
        return Ok(children);
    } else {
        return Err(ProcessError::NoChildProcesses.into());
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.mode {
        Subcommand::Propagate(Propagate {
            signal,
            depth,
            keep_alive,
        }) => {
            if keep_alive {
                loop {
                    match propagate_signal_to_all_child_pids(opts.pid as i32, depth, signal) {
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
                propagate_signal_to_all_child_pids(opts.pid as i32, depth, signal).unwrap();
            }
        }
    }
}

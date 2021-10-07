use crate::process::{get_child_pids_at_depth, get_child_pids_full_tree};
use crate::listener::SignalListener;
use crate::types::{Result, ChildPids, SignalMap};
use crate::traits::Listen;


use nix::sys::signal::kill;
use nix::unistd::Pid;



pub fn propagate_signal_to_all_child_pids(pid: u32, depth: u8, signal_map: SignalMap) -> Result<()> {
    let target_pids = get_target_pids(pid, depth)?;

    let mut join_handles: Vec<std::thread::JoinHandle<_>> = vec![];
    
    for pid in target_pids.into_iter() {
        let handle = std::thread::spawn(move || {
            propagate_signal(pid.clone(), signal_map)
        });
        join_handles.push(handle);
    }
    for handle in join_handles.into_iter() {
        handle.join().unwrap()?;
    }
    Ok(())
}

fn propagate_signal(pid: u32, signal_map: SignalMap) -> Result<()> {
    let listener = SignalListener::new(pid, signal_map.listen_signal);
    listener.listen()?;

    let child_pids = get_child_pids_full_tree(pid, None, None)?;
    for i in 1..=child_pids.len() as u8 {
        for pid in child_pids.get(&(i as usize)).unwrap().iter() {
            let send_signal = signal_map.send_signal();
            if let Ok(_) = kill(Pid::from_raw(*pid as i32), send_signal) {
                println!("Sent {} signal to child process: {} successfully", send_signal, pid);
            }
            else {
                println!("Unable to kill child process: {}", pid);
            }
            
        }
    }
    Ok(())
}

fn get_target_pids(pid: u32, depth: u8) -> Result<ChildPids> {
    if depth == 0 {
        return Ok(vec![pid]);
    }

    loop {
        match get_child_pids_at_depth(pid, depth) {
            Ok(Some(children)) if children.len() > 0 => return Ok(children),
            _ => continue
        }
    }
}
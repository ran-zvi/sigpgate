use crate::types::{ChildPids, Result};
use nix::{
    sys::wait::waitpid,
    unistd::{fork, getpid, ForkResult},
};
use std::{
    collections::HashMap,
    process::Command,
    str::FromStr,
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

pub fn get_child_pids(pid: i32) -> Result<Vec<i32>> {
    let output = Command::new("pgrep")
        .arg("-P")
        .arg(format!("{}", pid))
        .output();

    match output {
        Ok(o) => Ok(String::from_utf8(o.stdout)?
            .lines()
            .map(|s| i32::from_str(s).unwrap())
            .collect::<Vec<i32>>()),
        Err(e) => Err(e.into()),
    }
}

pub fn get_child_pids_full_tree<'a>(
    pid: i32,
    process_level_map: Option<&'a mut HashMap<usize, ChildPids>>,
    current_depth: Option<usize>
) -> Result<HashMap<usize, ChildPids>> {
    let mut new_map = HashMap::new();
    let mut process_level_map: &mut HashMap<_, _> = match process_level_map {
        None => &mut new_map,
        _ => process_level_map.unwrap(),
    };
    let current_depth = match current_depth {
        Some(n) => n,
        None => 1
    };
    

    let child_pids = get_child_pids(pid)?;

    if child_pids.len() == 0 {
        return Ok(process_level_map.clone());
    }

    process_level_map.entry(current_depth).or_insert(vec![]);

    for cpid in child_pids {
        process_level_map.get_mut(&current_depth).unwrap().push(cpid);
        get_child_pids_full_tree(cpid, Some(&mut process_level_map), Some(current_depth+1))?;
    }
    return Ok(process_level_map.clone());
}

pub fn get_child_pids_at_depth(pid: i32, depth: u8) -> Result<Option<ChildPids>> {
    let child_pids_by_depth: HashMap<_, _> = get_child_pids_full_tree(pid, None, None)?;
    let child_pids_at_depth = child_pids_by_depth.get(&(depth as usize));
    match child_pids_at_depth {
        Some(pids) => Ok(Some(pids.to_vec())),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    #[test]
    #[serial]
    fn test_get_child_pids_once() {
        let pid = getpid();

        let children = Arc::new(Mutex::new(vec![]));
        let mut handles = vec![];

        for _ in 0..5 {
            let children = Arc::clone(&children);
            let handle = spawn(move || {
                let mut child = Command::new("sleep")
                .arg("0.1")
                .spawn()
                .unwrap();

                children.lock().unwrap().push(child.id() as i32);
                child.wait().unwrap();
            });
            handles.push(handle);
        }

        let child_pids = get_child_pids(pid.into()).unwrap().sort();

        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(child_pids, children.lock().unwrap().sort());
    }

    #[test]
    #[serial]
    fn test_get_child_pids_full_tree() {
        let pid = getpid();

        let mut handles = vec![];


        for _ in 0..5 {
            let handle = spawn(move || {
                let mut child = Command::new("sleep")
                .arg("0.1")
                .spawn()
                .unwrap();
                child.wait().unwrap();
            });
            handles.push(handle);
        }

        let child_pids = get_child_pids_full_tree(pid.into(), None, None).unwrap();

        for handle in handles {
            handle.join().unwrap();
        }
        
        let depths: Vec<&usize> = child_pids.keys().collect();
        let processes_num = child_pids.values().collect::<Vec<_>>();
        
        assert_eq!(depths, vec![&0]);
        assert_eq!(processes_num.first().unwrap().len(), 5);
    }
}
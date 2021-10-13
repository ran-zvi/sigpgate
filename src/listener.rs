use nix::sys::{
    ptrace,
    ptrace::*,
    signal::{Signal, kill},
    wait::{waitpid, WaitStatus},
};
use nix::unistd::Pid;

use crate::types::{ListenStatus, Result};
use crate::traits::Listen;

const WAIT_INTERVAL_MS: u64 = 100;

pub struct SignalListener {
    pid: Pid,
    signal: Signal,
    max_wait_time: u64
}

impl SignalListener {
    pub fn new(pid: u32, signal: Signal, max_wait_time: u64) -> Self {
        let pid = Pid::from_raw(pid as i32);
        SignalListener { pid, signal, max_wait_time }
    }

   
    fn attach_to_process(&self) -> Result<()> {
        ptrace::attach(self.pid)?;
        println!("Attached to {}", self.pid);
        waitpid(self.pid, None)?;
        ptrace::setoptions(
            self.pid,
            Options::PTRACE_O_TRACESYSGOOD | Options::PTRACE_O_TRACEEXEC,
        )?;
        Ok(())
    }

    
}

pub fn wait_until_process_starts(pid: Pid, max_wait_time: u64) -> Result<()> {
    let mut time_elapsed = 0;
    loop {
        match kill(pid, None) {
            Ok(_) => return Ok(()),
            Err(e) => {
                time_elapsed += WAIT_INTERVAL_MS;
                if time_elapsed >= max_wait_time {
                    return Err(e.into());
                }
                std::thread::sleep(std::time::Duration::from_millis(WAIT_INTERVAL_MS));
            }
        }
    }
}

impl Listen for SignalListener {
    fn listen(&self) -> Result<ListenStatus> {
        println!("Listening for signal: {} on pid: {}", self.signal, self.pid);
        wait_until_process_starts(self.pid, self.max_wait_time)?;
        self.attach_to_process()?;

        loop {
            ptrace::syscall(self.pid, None)?;
            let status: WaitStatus = waitpid(self.pid, None)?;
            let signal = parse_status(status);

            if let Some(s) = signal {
                if s == self.signal {
                    break;
                }
            }
        }
        println!("Signal found!: {:?}", self.signal);
        Ok(ListenStatus::Found)
    }

}

fn parse_status(status: WaitStatus) -> Option<Signal> {
    match status {
        WaitStatus::Signaled(_, s, _) => Some(s),
        WaitStatus::Stopped(_, s) => Some(s),
        WaitStatus::PtraceEvent(_, s, _) => Some(s),
        _ => None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_parse_status_some() {
        let mut signals: Vec<Signal> = vec![];

        let status_1 = WaitStatus::Signaled(Pid::from_raw(0), Signal::SIGTERM, true);
        let status_2 = WaitStatus::Stopped(Pid::from_raw(0), Signal::SIGTERM);
        let status_3 = WaitStatus::PtraceEvent(Pid::from_raw(0), Signal::SIGTERM, 0);

        signals.push(parse_status(status_1).unwrap());
        signals.push(parse_status(status_2).unwrap());
        signals.push(parse_status(status_3).unwrap());

        for signal in signals.iter() {
            assert_eq!(*signal, Signal::SIGTERM);
        }
    }

    #[test]
    fn test_parse_status_none() {
        let status = WaitStatus::StillAlive;
        let signal = parse_status(status);
        assert_eq!(signal, None);
    }

    #[test]
    #[serial]
    fn test_listen_to_pid() {
        use nix::sys::signal;

        let mut child = std::process::Command::new("sleep")
            .arg("60")
            .spawn()
            .unwrap();
        std::thread::spawn(move || {});
        let pid = Pid::from_raw(child.id() as i32);

        let listener = SignalListener::new(child.id(), Signal::SIGHUP, 1000);
        let status = Arc::new(Mutex::new(None));
        let t_status = Arc::clone(&status);

        let handle = std::thread::spawn(move || {
            let mut s = t_status.lock().unwrap();
            *s = Some(listener.listen().unwrap());
        });
        std::thread::sleep(std::time::Duration::from_secs(1));
        signal::kill(pid, Signal::SIGHUP).unwrap();
        handle.join().unwrap();
        child.kill().unwrap();

        assert_eq!(*status.lock().unwrap(), Some(ListenStatus::Found));
    }

    #[test]
    #[should_panic(expected = "ESRCH: No such process")]
    fn test_pid_not_exist() {
        let pid = 99999999;
        let listener = SignalListener::new(pid, Signal::SIGHUP, 1000);
        listener.listen().unwrap();
    }
}

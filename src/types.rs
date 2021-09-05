use nix::sys::signal::Signal;

pub type Result<T> = anyhow::Result<T>;
pub type ChildPids = Vec<i32>;

#[derive(Debug, PartialEq)]
pub enum ListenStatus {
    Found,
    NotFound
}

#[derive(Clone, Copy)]
pub struct SignalMap {
    pub listen_signal: Signal,
    send_signal: Option<Signal>
}

impl SignalMap {

    pub fn new(listen_signal: Signal, send_signal: Option<Signal>) -> Self {
        SignalMap {
            listen_signal,
            send_signal: send_signal
        }
    }

    pub fn send_signal(&self) -> Signal {
        if let Some(signal) = self.send_signal {
            signal
        }
        else {
            self.listen_signal
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signalmap_only_listen() {
        let signal_map = SignalMap::new(Signal::SIGTERM, None);
        assert_eq!(signal_map.send_signal(), Signal::SIGTERM);
    }

    #[test]
    fn test_signalmap() {
        let signal_map = SignalMap::new(Signal::SIGTERM, Some(Signal::SIGKILL));
        assert_eq!(signal_map.send_signal(), Signal::SIGKILL);
    }
}
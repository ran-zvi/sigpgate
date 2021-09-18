# sigpgate
Signal Propagator

Sigpgate uses `ptrace` to attach to a process and listen for status changes, it then propagates signals down the hierarchy of a process tree.

## Use cases
- SIGTERM (or any signal of your choice) which doesn't propagate down the hierarchy of a process tree all the way down to a child can now be forced to propagate.
- Signal boosting, for when you wish that SIGTERM was a SIGKILL, but don't want to modify your code. 

## USAGE

```bash
sigpgate <pid>
```

### Flags
- `-v --verbose`: verbose - default: false
- `-l --listen-signal`: signal type to listen to - default: `SIGTERM`.
- `-s --send-signal`: signal type to send after listen signal was found - default: <`-l` flag value>.
- `-d --depth`: depth of the child process you wish to listen to signals on, e.g `-d 2` will listen to a signal on all grandchildren of the `pid` specified.
- `-k --keep-alive`: keep the program running even after a signal was found, useful for when a process continuosly creates children.


## Example

Assume the following process tree:
```bash
|--- 1
|   |---2
|   |   |---3
|   |---4
|   |   |---5
```

`sigpgate 1 -d 1` will listen to `SIGTERM` signal on pids: `2`, `4`. When it's found it will send a SIGTERM to pids: `3`, `5`


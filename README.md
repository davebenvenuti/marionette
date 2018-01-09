# marionette
Command line app that runs bash commands on one or more remote machines reachable via ssh, authenticating with your locally running ssh-agent

## Requirements

### General
- libssh2

### Development
- Rust v.?
- cmake

## Usage

```bash
USAGE:
    marionette --command <COMMAND> --host <HOST> --username <USERNAME>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --command <COMMAND>      Command to issue
    -h, --host <HOST>            One or more hosts, comma delimited, eg: server1.com:22,server2.com:22
    -u, --username <USERNAME>    Username to ssh as 
```

If running from a git checkout, run with `cargo run` instead of `marionette`

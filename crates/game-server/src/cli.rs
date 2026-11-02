//! Hand-rolled CLI parser.
//!
//! Deliberately dependency-free: the flag surface is small and stable, and a
//! server binary that pulls a proc-macro tree to read eight strings is worse
//! than sixty lines of matching. Supports `--flag value` and `--flag=value`.

use std::fmt;
use std::net::SocketAddr;
use std::path::PathBuf;

use crate::log::Level;
use crate::Config;

#[derive(Debug)]
pub struct CliError(pub String);

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CliError {}

pub enum Command {
    Run(Box<Config>),
    Help,
    Version,
}

pub const USAGE: &str = "\
game-server — headless server for the recovered wasm64 GTA V engine port

USAGE:
    game-server [OPTIONS]

OPTIONS:
    --data-dir <path>      Asset/userdata root (default: ./userdata)
    --tick-hz <hz>         Fixed simulation rate, 1..=2000 (default: 30)
    --threads <n>          Worker threads for host dispatch, 1..=256 (default: 1)
    --listen <addr:port>   Control listener (default: 127.0.0.1:7777, use :0 for ephemeral)
    --max-clients <n>      Concurrent control clients, 1..=1024 (default: 8)
    --log-level <lvl>      error|warn|info|debug|trace (default: info)
    --headless             Run without a renderer (default, and required here)
    --single-threaded      No worker pool; tick + control threads only
    --max-ticks <n>        Exit after n ticks (test hook, default: run forever)
    --memory-mb <mb>       Linear-memory reservation in MiB (default: 3072)
    -h, --help             Print this help
    -V, --version          Print version

STATUS:
    Send one line per command to the control port:
        status   -> one-line JSON with frame, tick time, uptime, peers, RSS
        quit     -> graceful shutdown
";

pub fn parse<I>(args: I) -> Result<Command, CliError>
where
    I: IntoIterator<Item = String>,
{
    let mut cfg = Config::default();
    let mut iter = args.into_iter().peekable();

    while let Some(arg) = iter.next() {
        // Split `--flag=value` once, up front.
        let (name, inline) = match arg.split_once('=') {
            Some((n, v)) if n.starts_with("--") => (n.to_string(), Some(v.to_string())),
            _ => (arg.clone(), None),
        };

        let mut take_value = |what: &str| -> Result<String, CliError> {
            if let Some(v) = &inline {
                return Ok(v.clone());
            }
            iter.next().ok_or_else(|| {
                CliError(format!("flag {what} needs a value (try --help)"))
            })
        };

        match name.as_str() {
            "-h" | "--help" => return Ok(Command::Help),
            "-V" | "--version" => return Ok(Command::Version),
            "--headless" => {
                cfg.headless = inline.as_deref().map(parse_bool).transpose()?.unwrap_or(true);
            }
            "--no-headless" => cfg.headless = false,
            "--single-threaded" => {
                cfg.single_threaded =
                    inline.as_deref().map(parse_bool).transpose()?.unwrap_or(true);
            }
            "--data-dir" => {
                let v = take_value("--data-dir")?;
                if v.trim().is_empty() {
                    return Err(CliError("--data-dir must not be empty".into()));
                }
                cfg.data_dir = PathBuf::from(v);
            }
            "--tick-hz" => {
                let v = take_value("--tick-hz")?;
                cfg.tick_hz = parse_num(&v, "--tick-hz")?;
                if !(1..=2000).contains(&cfg.tick_hz) {
                    return Err(CliError(format!(
                        "--tick-hz must be between 1 and 2000, got {}",
                        cfg.tick_hz
                    )));
                }
            }
            "--threads" => {
                let v = take_value("--threads")?;
                let n: usize = parse_num(&v, "--threads")?;
                if n == 0 || n > 256 {
                    return Err(CliError(format!(
                        "--threads must be between 1 and 256, got {n}"
                    )));
                }
                cfg.threads = n;
            }
            "--max-clients" => {
                let v = take_value("--max-clients")?;
                let n: usize = parse_num(&v, "--max-clients")?;
                if n == 0 || n > 1024 {
                    return Err(CliError(format!(
                        "--max-clients must be between 1 and 1024, got {n}"
                    )));
                }
                cfg.max_clients = n;
            }
            "--listen" => {
                let v = take_value("--listen")?;
                cfg.listen = parse_addr(&v)?;
            }
            "--log-level" => {
                let v = take_value("--log-level")?;
                cfg.log_level = Level::parse(&v)
                    .ok_or_else(|| CliError(format!("unknown --log-level {v:?}")))?;
            }
            "--max-ticks" => {
                let v = take_value("--max-ticks")?;
                let n: u64 = parse_num(&v, "--max-ticks")?;
                if n == 0 {
                    return Err(CliError("--max-ticks must be greater than 0".into()));
                }
                cfg.max_ticks = Some(n);
            }
            "--memory-mb" => {
                let v = take_value("--memory-mb")?;
                let mb: u64 = parse_num(&v, "--memory-mb")?;
                if mb < 1 || mb > 65536 {
                    return Err(CliError(format!(
                        "--memory-mb must be between 1 and 65536, got {mb}"
                    )));
                }
                cfg.memory_bytes = mb * 1024 * 1024;
            }
            other => {
                return Err(CliError(format!(
                    "unknown flag {other:?} (try --help)"
                )))
            }
        }
    }

    Ok(Command::Run(Box::new(cfg)))
}

fn parse_num<T: std::str::FromStr>(raw: &str, flag: &str) -> Result<T, CliError> {
    raw.trim()
        .parse::<T>()
        .map_err(|_| CliError(format!("{flag} expects a number, got {raw:?}")))
}

fn parse_bool(raw: &str) -> Result<bool, CliError> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Ok(true),
        "0" | "false" | "no" | "off" => Ok(false),
        other => Err(CliError(format!("expected a boolean, got {other:?}"))),
    }
}

fn parse_addr(raw: &str) -> Result<SocketAddr, CliError> {
    raw.trim().parse::<SocketAddr>().map_err(|_| {
        CliError(format!(
            "--listen expects host:port (e.g. 127.0.0.1:7777 or [::1]:7777), got {raw:?}"
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Result<Command, CliError> {
        parse(list.iter().map(|s| s.to_string()))
    }

    fn run(list: &[&str]) -> Config {
        match args(list).expect("should parse") {
            Command::Run(c) => *c,
            _ => panic!("expected run command"),
        }
    }

    #[test]
    fn defaults_are_sane() {
        let cfg = run(&[]);
        assert_eq!(cfg.tick_hz, 30);
        assert_eq!(cfg.threads, 1);
        assert_eq!(cfg.max_clients, 8);
        assert!(cfg.headless);
        assert_eq!(cfg.memory_bytes, 3 * 1024 * 1024 * 1024);
        assert_eq!(cfg.data_dir, PathBuf::from("./userdata"));
        assert!(cfg.max_ticks.is_none());
    }

    #[test]
    fn accepts_both_value_forms() {
        let cfg = run(&["--tick-hz=60", "--listen", "0.0.0.0:9000", "--log-level", "debug"]);
        assert_eq!(cfg.tick_hz, 60);
        assert_eq!(cfg.listen.port(), 9000);
        assert_eq!(cfg.log_level, Level::Debug);
    }

    #[test]
    fn rejects_bad_numbers() {
        assert!(args(&["--tick-hz", "0"]).is_err());
        assert!(args(&["--tick-hz", "abc"]).is_err());
        assert!(args(&["--threads", "0"]).is_err());
        assert!(args(&["--max-clients", "0"]).is_err());
    }

    #[test]
    fn rejects_missing_and_unknown() {
        assert!(args(&["--data-dir"]).is_err());
        assert!(args(&["--wat"]).is_err());
    }

    #[test]
    fn help_and_version_are_commands() {
        assert!(matches!(args(&["--help"]).unwrap(), Command::Help));
        assert!(matches!(args(&["-V"]).unwrap(), Command::Version));
    }

    #[test]
    fn single_threaded_is_reflected() {
        assert!(run(&["--single-threaded"]).single_threaded);
    }
}
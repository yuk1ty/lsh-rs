use lsh::common::*;
use std::io;
use std::ffi::CString;
use std::path::*;
use std::str::FromStr;
use std::fmt;
use nix::unistd::*;
use nix::sys::wait::*;

pub mod common;

pub struct LshLoop;

impl LshLoop {
    pub fn new() -> LshLoop {
        LshLoop
    }

    pub fn start(&self) -> Result<Status, LshError> {
        loop {
            let mut s = String::new();
            let scan = io::stdin();
            let _ = scan.read_line(&mut s);
            let args: Vec<String> = s.split_whitespace()
                .map(|token: &str| String::from(token))
                .collect();

            match execute(args) {
                Ok(status) => match status {
                    Status::Success => continue,
                    exit @ Status::Exit => return Ok(exit),
                },
                err @ Err(_) => return err,
            };
        }
    }
}

pub enum Commands {
    Cd,
    Help,
    Exit,
    Execute,
}

impl Commands {
    fn values() -> Vec<String> {
        vec![
            Commands::Cd.to_string(),
            Commands::Help.to_string(),
            Commands::Exit.to_string(),
            Commands::Execute.to_string(),
        ]
    }
}

impl FromStr for Commands {
    type Err = LshError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "cd" {
            Ok(Commands::Cd)
        } else if s == "help" {
            Ok(Commands::Help)
        } else if s == "exit" {
            Ok(Commands::Exit)
        } else {
            Ok(Commands::Execute)
        }
    }
}

impl fmt::Display for Commands {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(println!(
            "{}",
            match *self {
                Commands::Cd => "cd",
                Commands::Help => "help",
                Commands::Exit => "exit",
                Commands::Execute => "execute",
            }
        ))
    }
}

/// Execute shell build-in or launch program.
fn execute(args: Vec<String>) -> Result<Status, LshError> {
    if args.len() == 0 {
        return Err(LshError::new("Empty command"));
    }

    Commands::from_str(&args[0]).and_then(|c: Commands| match c {
        Commands::Cd => lsh_cd(&args[1]),
        Commands::Help => lsh_help(),
        Commands::Exit => lsh_exit(),
        Commands::Execute => lsh_launch(args),
    })
}

fn lsh_cd(dir: &str) -> Result<Status, LshError> {
    if dir.is_empty() {
        Err(LshError::new("lsh: expected argument to cd\n"))
    } else {
        chdir(Path::new(&dir))
            .map(|_| Status::Success)
            .map_err(|err| LshError::new(&err.to_string()))
    }
}

fn lsh_help() -> Result<Status, LshError> {
    println!("Yuki Toyoda's LSH inspired by lsh");
    println!("Type program names and arguments, and hit enter.");
    println!("The following are build in:");
    println!("Use the man command for information on other programs.");
    Ok(Status::Success)
}

fn lsh_exit() -> Result<Status, LshError> {
    Ok(Status::Exit)
}

fn lsh_launch(args: Vec<String>) -> Result<Status, LshError> {
    let pid = fork().map_err(|_| LshError::new("fork failed"))?;
    match pid {
        ForkResult::Parent { child } => loop {
            let wait_pid_result = waitpid(child, None).map_err(|_| LshError::new("failed to waitpid."));
            let result = match wait_pid_result {
                Ok(WaitStatus::Exited(_, _)) => Ok(Status::Success),
                Ok(WaitStatus::Signaled(_, _, _)) => Ok(Status::Success),
                _ => Err(LshError::new("EXITED")),
            };

            if result.is_err() {
                return result;
            }
        },
        ForkResult::Child => {
            let path = CString::new(args[1].to_string()).unwrap();
            let args = CString::new(args[2].to_string()).unwrap();
            execv(&path, &[path.clone(), args])
                .map(|_| Status::Success)
                .map_err(|_| LshError::new("Child Process failed"))
        }
    }
}

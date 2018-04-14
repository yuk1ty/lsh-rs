use std::env;
use lsh::common::*;

pub mod common;

pub struct LshLoop;

impl LshLoop {
    pub fn new() -> LshLoop {
        LshLoop
    }

    pub fn start(&self) -> Result<Status, Box<LshError>> {
        loop {
            print!("> ");
            let args: Vec<String> = env::args().collect();
            match execute(args) {
                Ok(status) => match status {
                    Status::Success => continue,
                    exit @ Status::Exit => return Ok(exit)
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
}

/// Execute shell build-in or launch program.
fn execute(args: Vec<String>) -> Result<Status, Box<LshError>> {
    if args.len() < 2 {
        return Err(Box::new(LshError::new("Empty Command.")));
    }

    match args[0].as_ref() {
        "cd" => Ok(Status::Success),
        "help" => Ok(Status::Success),
        "exit" => Ok(Status::Success),
        _ => Err(Box::new(LshError::new("non-supported command."))),
    }
}

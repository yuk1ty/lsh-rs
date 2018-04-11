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
            execute(args)?;
        }
    }
}

fn execute(args: Vec<String>) -> Result<Status, Box<LshError>> {
    if args.len() < 2 {
        return Err(Box::new(LshError::new("Empty Command.")));
    }

    Ok(Status::Success)
}

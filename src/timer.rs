use std::time::{Duration, Instant};

// TODO Hierarchy is lost by linearizing
// TODO Color or something for intermediate sections, and move logging statements around a bit
// TODO Print final summary better

// TODO API styles...
//
// 1) start(x) and stop(x)
// 2) let x = start(x); .... x.stop()
// 3) timer.block("foo", { ... })

pub struct Timer {
    done: Vec<FinishedBlock>,
    stack: Vec<CurrentBlock>,
}

struct CurrentBlock {
    name: String,
    started: Instant,
    level: usize,
}

struct FinishedBlock {
    name: String,
    duration: Duration,
    level: usize,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            done: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn start<I: Into<String>>(&mut self, name: I) {
        let block = CurrentBlock {
            level: self.stack.len(),
            name: name.into(),
            started: Instant::now(),
        };
        println!("{} {}", "##".repeat(block.level + 1), block.name);
        self.stack.push(block);
    }

    // TODO Take an additional message, "got xyz things"
    pub fn stop(&mut self) {
        let block = self.stack.pop().unwrap();
        let done = FinishedBlock {
            level: block.level,
            name: block.name,
            duration: Instant::now().duration_since(block.started),
        };
        println!(
            "{} {} took {:?}",
            "##".repeat(done.level + 1),
            done.name,
            done.duration
        );
        self.done.push(done);
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        assert!(self.stack.is_empty());
        // TODO Formatted like a flamegraph? Emphasize the proportionally expensive sections
        println!("\n\n\nSummary:");
        for block in &self.done {
            println!(
                "{}- {} took {:?}",
                "  ".repeat(block.level),
                block.name,
                block.duration
            );
        }
    }
}

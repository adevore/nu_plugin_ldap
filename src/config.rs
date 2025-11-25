use std::time::Duration;

#[derive(Debug)]
struct Config {
    timeout: Duration,
}

impl Config {
    fn new(timeout: Duration) -> Self {
        Config { timeout }
    }
}

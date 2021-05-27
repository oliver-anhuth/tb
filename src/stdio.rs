//! Simpler standard io

// Standard Input
pub struct StdIn {
    buffer: Vec<String>,
}

impl StdIn {
    pub fn new() -> StdIn {
        StdIn { buffer: Vec::new() }
    }
    // Adapted from https://codeforces.com/contest/1168/submission/54903799
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed to parse input token");
            }

            let mut line = String::new();
            std::io::stdin()
                .read_line(&mut line)
                .expect("Failed to read from stdin");
            self.buffer
                .append(&mut line.split_whitespace().rev().map(String::from).collect());
        }
    }
}

// Standard Output
pub struct StdOut {
    writer: std::io::BufWriter<std::io::Stdout>,
}

impl StdOut {
    pub fn new() -> StdOut {
        StdOut {
            writer: std::io::BufWriter::new(std::io::stdout()),
        }
    }
}

impl std::ops::Deref for StdOut {
    type Target = std::io::BufWriter<std::io::Stdout>;
    fn deref(&self) -> &std::io::BufWriter<std::io::Stdout> {
        &self.writer
    }
}

impl std::ops::DerefMut for StdOut {
    fn deref_mut(&mut self) -> &mut std::io::BufWriter<std::io::Stdout> {
        &mut self.writer
    }
}

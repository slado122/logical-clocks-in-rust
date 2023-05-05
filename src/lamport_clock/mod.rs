#![allow(dead_code)]

#[cfg(test)]
mod tests;

pub struct Node {
    pub id: usize,
    lamport_clock: LamportClock,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            lamport_clock: LamportClock { latest_time: 1 },
        }
    }

    pub fn send(&mut self, other: &mut Node) {
        let request_time = self.lamport_clock.latest_time;
        let response_time = other.lamport_clock.tick(request_time);
        self.lamport_clock.tick(response_time);
    }
}

struct LamportClock {
    latest_time: usize,
}

impl LamportClock {
    fn tick(&mut self, request_time: usize) -> usize {
        self.latest_time = std::cmp::max(self.latest_time, request_time) + 1;
        self.latest_time
    }
}

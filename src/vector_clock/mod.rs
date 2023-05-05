#![allow(dead_code)]

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct VectorClock {
    clock: Vec<usize>,
}

impl VectorClock {
    fn new(nodes_count: usize) -> VectorClock {
        Self {
            clock: vec![0; nodes_count],
        }
    }

    fn increment(&mut self, index: usize) {
        self.clock[index] += 1;
    }

    fn merge(&mut self, other: &VectorClock) {
        assert_eq!(self.clock.len(), other.clock.len());
        for i in 0..self.clock.len() {
            if self.clock[i] < other.clock[i] {
                self.clock[i] = other.clock[i];
            }
        }
    }
}

impl PartialEq for VectorClock {
    fn eq(&self, other: &VectorClock) -> bool {
        self.clock == other.clock
    }
}

impl PartialOrd for VectorClock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        assert_eq!(self.clock.len(), other.clock.len());

        let mut less_than = false;
        let mut greater_than = false;
        for i in 0..self.clock.len() {
            if self.clock[i] < other.clock[i] {
                less_than = true;
            } else if self.clock[i] > other.clock[i] {
                greater_than = true;
            }
        }
        match (less_than, greater_than) {
            (true, true) => None,
            (true, false) => Some(std::cmp::Ordering::Less),
            (false, true) => Some(std::cmp::Ordering::Greater),
            (false, false) => Some(std::cmp::Ordering::Equal),
        }
    }
}

use std::cell::RefCell;

pub struct Variations {
    state: RefCell<Vec<usize>>,
    char_set: Vec<String>,
    n: usize, // length of the char_set
    m: usize, // capacity of the state

    initial: bool,
}

impl Variations {
    pub fn new(char_set: Vec<String>, m: usize) -> Self {
        let mut s = Vec::new();
        s.resize_with(m, || 0);
        Variations {
            n: char_set.len(),
            m: m,
            state: RefCell::new(s),
            char_set: char_set,
            initial: true,
        }
    }

    fn map_state(&self) -> Option<String> {
        let mut result = Vec::<String>::new();
        for v in self.state.borrow_mut().clone().into_iter() {
            result.push(self.char_set[v].clone().into());
        }

        Some(result.join(""))
    }
}

impl Iterator for Variations {
    type Item = String;

    // next using a specific algorithm
    // 1. increment the 0 position
    // 2. if the 0 pos is greater or equal with the char len then
    // 3a. increment the next position, and if it's greater, then
    // 4a. increment the next position and continue this
    // 3b. if the next position is the end of the state then return None
    fn next(&mut self) -> Option<String> {
        let pos = 0;
        if self.initial {
            self.initial = false;
            return self.map_state();
        }

        self.state.borrow_mut()[pos] += 1;

        if self.state.borrow()[pos] >= self.n {
            let mut counter = 1;
            loop {
                if pos + counter >= self.m {
                    return None;
                }
                self.state.borrow_mut()[pos] = 0;
                self.state.borrow_mut()[pos + counter] += 1;
                if self.state.borrow()[pos + counter] >= self.n {
                    self.state.borrow_mut()[pos + counter] = 0;
                    counter += 1;
                    continue;
                } else {
                    break;
                }
            }
        }

        self.map_state()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_alg() {
        let split = "abcde".split("");
        let mut available_chars: Vec<String> =
            split.map(|s| String::from(s)).collect::<Vec<String>>();
        available_chars.drain(0..1);
        available_chars.drain(available_chars.len() - 1..);
        println!("{:?}", available_chars);

        let v = Variations::new(available_chars, 3);
        match v.map_state() {
            Some(s) => println!("{}", s),
            None => {}
        };
    }

    #[test]
    fn test_next() {
        let split = "abcde".split("");
        let mut available_chars: Vec<String> =
            split.map(|s| String::from(s)).collect::<Vec<String>>();
        available_chars.drain(0..1);
        available_chars.drain(available_chars.len() - 1..);
        println!("{:?}", available_chars);

        let mut v = Variations::new(available_chars, 3);
        for _ in 1..10 {
            match v.next() {
                Some(s) => println!("{}", s),
                None => {}
            };
        }
    }

    #[test]
    fn test_permutation() {
        let results = [
            "aaa", "baa", "caa", "aba", "bba", "cba", "aca", "bca", "cca", "aab", "bab", "cab",
            "abb", "bbb", "cbb", "acb", "bcb", "ccb", "aac", "bac", "cac", "abc", "bbc", "cbc",
            "acc", "bcc", "ccc",
        ];

        let split = "abc".split("");
        let mut available_chars: Vec<String> =
            split.map(|s| String::from(s)).collect::<Vec<String>>();
        available_chars.drain(0..1);
        available_chars.drain(available_chars.len() - 1..);
        println!("{:?}", available_chars);

        let mut v = Variations::new(available_chars, 3);
        let mut counter = 0;
        loop {
            match v.next() {
                Some(s) => {
                    if s != String::from(results[counter]) {
                        panic!(format!(
                            "Counter: {}, Result: {}, Variation: {}",
                            counter, results[counter], s
                        ));
                    }
                }
                None => break,
            };

            counter += 1;
        }
    }
}

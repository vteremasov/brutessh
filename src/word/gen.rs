use std::cell::RefCell;

pub struct Variations {
    state: RefCell<Vec<usize>>,
    char_set: Vec<String>,
    n: usize, // length of the char_set

    position: usize, // position counter of the state
}

impl Variations {
    pub fn new(char_set: Vec<String>, m: usize) -> Self {
        let mut s = Vec::new();
        s.resize_with(m, || 0);
        Variations {
            n: char_set.len(),
            state: RefCell::new(s),
            char_set: char_set,
            position: 0,
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

    fn next(&mut self) -> Option<String> {
        if self.state.borrow_mut()[self.position] >= self.n - 1 {
            self.state.borrow_mut()[self.position] = 0;
            self.position += 1;
        }

        self.state.borrow_mut()[self.position] += 1;

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
}

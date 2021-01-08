use crate::fibonacci::*;

pub struct FibGenerator {
    fib_index: usize,
    index: usize,
    size: usize,
    fibonaccis: Vec<Fibonacci>,
}

impl FibGenerator {
    pub fn new() -> FibGenerator {
        let size = 0;
        let fib_index = 0;
        let index = 0;
        let fibonaccis = Vec::new();
        FibGenerator {
            size,
            fibonaccis,
            fib_index,
            index,
        }
    }
    pub fn iterate(&mut self) {
        self.fib_index = 0;
        self.index = 0;
        for i in 0..self.size {
            self.fibonaccis[i].iterate();
        }
    }
    fn add_fibonacci(&mut self, size: usize) {
        self.fibonaccis.push(Fibonacci::new(size));
        self.size += 1;
    }
    pub fn generate(&mut self) -> usize {
        if self.size <= self.fib_index {
            self.add_fibonacci(self.fib_index + 2);
        }
        let number = self.fibonaccis[self.fib_index].number(self.index);
        self.index += 1;
        if self.index >= self.fibonaccis[self.fib_index].size() {
            self.index = 0;
            self.fib_index += 1;
        }
        number
    }
}

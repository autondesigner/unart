pub struct Fibonacci {
    size: usize,
    numbers: Vec<usize>,
}

impl Fibonacci {
    pub fn new(size: usize) -> Fibonacci {
        if size < 2 {
            panic!("fib size < 2");
        }
        let mut numbers = Vec::with_capacity(size);
        for _i in 0..size - 1 {
            numbers.push(0);
        }
        numbers.push(1);
        Fibonacci { size, numbers }
    }
    pub fn number(&self, index: usize) -> usize {
        self.numbers[index]
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn iterate(&mut self) {
        let mut next_fib = self.numbers[0];
        for i in 1..self.size {
            next_fib = next_fib.wrapping_add(self.numbers[i]);
        }
        for i in 0..self.size - 1 {
            self.numbers[i] = self.numbers[i + 1];
        }
        self.numbers[self.size - 1] = next_fib;
    }
}

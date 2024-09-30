struct CustomStack {
    buffer: Vec<i32>,
    increments: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {

    fn new(maxSize: i32) -> Self {
        Self {
            buffer: Vec::with_capacity(maxSize as usize),
            increments: vec![0; maxSize as usize],
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.buffer.len() < self.buffer.capacity() { self.buffer.push(x) }
    }
    
    fn pop(&mut self) -> i32 {
        match self.buffer.pop() {
            Some(result) => {
                let result = result + self.increments[self.buffer.len()];
                if !self.buffer.is_empty() { self.increments[self.buffer.len() - 1] += self.increments[self.buffer.len()] }
                self.increments[self.buffer.len()] = 0;
                result
            },
            None => -1
        }
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        if !self.buffer.is_empty() { self.increments[(self.buffer.len() - 1).min((k - 1) as usize)] += val }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */
pub struct RingBuffer<T> {
    contents: Vec<T>,
    tail: usize,
    size: usize,
}

impl<T: Clone> RingBuffer<T> {
    pub fn new(default: T, size: usize) -> Self {
        let contents = vec![default; size];
        let tail = 0usize;
        RingBuffer { contents, tail, size }
    }
    pub fn push(&mut self, value: T) {
        self.inc_tail();
        self.contents[self.tail] = value;
    }
    fn inc_tail(&mut self) {
        self.tail += 1;
        if self.tail == self.contents.len() {
            self.tail = 0;
        }
    }
    /// Get the element at tail - idx
    /// idx must be less than the size of the ring buffer
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.size {return None}
        let mut out = self.tail as isize - idx as isize;
        if out < 0 {
            out += self.contents.len() as isize;
        }
        Some(&self.contents[out as usize])
    }
}

pub struct RingBufferVec<T> {
    ring_buffer: RingBuffer<T>,
    size: usize,
}

impl<T: Clone> RingBufferVec<T> {
    pub fn new(default: T, size: usize) -> Self {
        RingBufferVec { ring_buffer: RingBuffer::new(default, size), size: 0 }
    }
    pub fn len(&self) -> usize { self.size }
    pub fn push(&mut self, value: T) {
        self.size += 1;
        self.ring_buffer.push(value);
    }
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx > self.size || (idx as isize) < self.size as isize -  self.ring_buffer.size as isize {return None}
        let mut idx = self.ring_buffer.tail as isize - idx as isize - 1;
        while idx < 0 {
            idx = self.ring_buffer.size as isize + idx;
        }
        
        self.ring_buffer.get(idx as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ring_buffer_test_push() {
        let mut rb = RingBuffer::<i32>::new(10, 10);
        rb.push(1);
        rb.push(2);
        rb.push(3);
        rb.push(4);
        rb.push(5);
        rb.push(6);
        rb.push(7);
        rb.push(8);
        rb.push(9);
        rb.push(10);
        rb.push(11);
        assert_eq!(rb.get(0), Some(&11));
        assert_eq!(rb.get(1), Some(&10));
        assert_eq!(rb.get(2), Some(&9));
        assert_eq!(rb.get(9), Some(&2));
    }
    #[test]
    fn ring_buffer_vec_push() {
        let mut rb = RingBufferVec::new(-69, 10);
        rb.push(1);
        rb.push(2);
        rb.push(3);
        rb.push(4);
        rb.push(5);
        rb.push(6);
        rb.push(7);
        rb.push(8);
        rb.push(9);
        assert_eq!(rb.get(0), Some(&1));
        assert_eq!(rb.get(1), Some(&2));
        assert_eq!(rb.get(2), Some(&3));
        assert_eq!(rb.get(9), Some(&-69));
        rb.push(10);
        rb.push(11);
        rb.push(12);
        assert_eq!(rb.get(9), Some(&10));
        assert_eq!(rb.get(10), Some(&11));
        assert_eq!(rb.get(11), Some(&12));
        assert_eq!(rb.get(0), None);
    }
    #[test]
    fn ring_buffer_vec_push2() {
        let mut rb = RingBufferVec::new(-69, 2);
        rb.push(0);
        rb.push(1);
        rb.get(0);
    }
}
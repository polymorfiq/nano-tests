pub struct Queue<T, const SIZE: usize> {
    pub(self) start_idx: usize,
    pub(self) size: usize,
    pub(self) items: [T; SIZE]
}

impl<T, const SIZE: usize> Queue<T, SIZE> {
    pub const fn new(start_val: [T; SIZE]) -> Self {
        Self {
            start_idx: 0,
            size: 0,
            items: start_val
        }
    }

    pub fn push(&mut self, event: T) -> Option<usize> {
        if self.size >= SIZE {
            None
        } else {
            self.items[Self::index_at(self.start_idx + self.size)] = event;
            self.size += 1;
            Some(self.size)
        }
    }

    pub fn pop(&mut self) -> Option<&T> {
        if self.size > 0 {
            let resp = &self.items[self.start_idx];
            self.size -= 1;
            self.start_idx = Self::index_at(self.start_idx + 1);
            Some(resp)
        } else {
            None
        }
    }

    pub const fn index_at(idx: usize) -> usize {
        if idx > SIZE {
            Self::index_at(idx - SIZE)
        } else {
            idx
        }
    }
}

impl<T, const SIZE: usize> Iterator for Queue<T, SIZE> where T: core::marker::Copy {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t_ptr) = self.pop() {
            Some(*t_ptr)
        } else {
            None
        }
    }
}
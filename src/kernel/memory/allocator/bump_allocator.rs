

pub struct BumpAllocator {
     pub start: usize,
     pub next: usize,
     pub end: usize
}

impl BumpAllocator {

    pub fn constructor(start: usize, end: usize) -> Self{ 
        Self { start, next: start, end}
    }

    pub fn alloc(&mut self, size: usize) -> Option<usize> { // выделяем память 
        let old_memory = self.next;

        if self.next + size > self.end {
            None
        } else {
            self.next += size;
            Some(old_memory)
        }
    }

    pub fn free(&mut self) { // сброс
        self.next = self.start;
    }
}
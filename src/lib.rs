use std::ptr::NonNull;

pub struct VectRust<T> {
    capacity: usize,
    len: usize,
    ptr: NonNull<T>,
}

impl<T> VectRust<T> {
    pub fn new() -> Self {
        Self {
            capacity: 0,
            len: 0,
            ptr: NonNull::dangling(),
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut vec = Vec::new();
        vec.push(1usize);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }
}

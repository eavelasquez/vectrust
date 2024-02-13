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
    use super::*;

    #[test]
    fn it_works() {
        let vec: VectRust<usize> = VectRust::new();

        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
    }
}

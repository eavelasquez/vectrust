use std::{
    alloc::{self, Layout},
    mem,
    ptr::NonNull,
};

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

    pub fn push(&mut self, value: T) {
        assert_ne!(mem::size_of::<T>(), 0, "No zero sized types");

        if self.capacity == 0 {
            let layout = Layout::array::<T>(4).expect("Could not allocate");

            // SAFETY: the layout is hardcoded to be 4 * size_of<T> and
            // size_of<T> is > 0.
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("Could not allocate memory");

            // SAFETY: ptr is non-null and we have  just allocated enough space for
            // this item (and 3 more). The memory previously at ptr is not read.
            unsafe { ptr.as_ptr().write(value) };

            self.ptr = ptr;
            self.len = 1;
            self.capacity = 4;
        } else if self.len <= self.capacity {
            todo!("Implement push for when capacity is not zero and len <= capacity")
        } else {
            todo!("Implement push for when capacity is not zero and len > capacity")
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
        let mut vec: VectRust<usize> = VectRust::new();
        vec.push(1usize);

        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.len(), 1);
    }
}

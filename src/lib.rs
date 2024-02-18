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
        } else if self.len < self.capacity {
            let offset = self
                .len
                .checked_mul(mem::size_of::<T>())
                .expect("Cannot reach memory location");
            assert!(offset < isize::MAX as usize, "Wrapped isize");

            // SAFETY: self.len is less than self.capacity, so we have space to write to.
            unsafe {
                // Offset cannot wrap around and pointer is pointing to valid memory
                // and writing to an offset at self.len is valid
                self.ptr.as_ptr().add(self.len).write(value);
            }
            self.len += 1;
        } else {
            debug_assert!(self.len == self.capacity);

            let size = std::mem::size_of::<T>() * self.capacity;
            let align = mem::align_of::<T>();

            size.checked_add(size % align).expect("Cannot allocate");

            let new_capacity = self.capacity.checked_mul(2).expect("Capacity wrapped");

            let ptr = unsafe {
                let layout = Layout::from_size_align_unchecked(size, align);
                let new_size = mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);

                let ptr = NonNull::new(ptr as *mut T).expect("Could not reallocate");
                ptr.as_ptr().add(self.len).write(value);

                ptr
            };

            self.ptr = ptr;
            self.len += 1;
            self.capacity = new_capacity;
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for VectRust<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let layout = Layout::from_size_align_unchecked(
                mem::size_of::<T>() * self.capacity,
                mem::align_of::<T>(),
            );
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec: VectRust<usize> = VectRust::new();
        vec.push(1usize);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }
}

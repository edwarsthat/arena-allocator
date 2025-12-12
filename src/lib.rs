mod errors;
use std::alloc::dealloc;
use std::mem;
use std::{
    alloc::{Layout, alloc},
    cell::Cell,
    ptr::NonNull,
};

use crate::errors::ArenaError;

pub struct Arena {
    buffer: NonNull<u8>,
    capacity: usize,
    offset: Cell<usize>,
    layout: Layout,
}

impl Arena {
    pub fn new(capacity: usize) -> Result<Self, ArenaError> {
        if capacity == 0 {
            return Err(ArenaError::ZeroCapacity);
        }

        let align = mem::align_of::<usize>();
        let layout = match Layout::from_size_align(capacity, align) {
            Ok(layout) => layout,
            Err(_) => return Err(ArenaError::InvalidLayout),
        };
        let ptr = unsafe { alloc(layout) };
        let buffer = match NonNull::new(ptr) {
            Some(non_null_ptr) => non_null_ptr,
            None => {
                return Err(ArenaError::AllocationFailed);
            }
        };

        Ok(Arena {
            buffer,
            capacity,
            offset: Cell::new(0),
            layout,
        })
    }

    pub fn alloc<T>(&self, value: T) -> &mut T {
        let size = mem::size_of::<T>();
        let align = mem::align_of::<T>();

        let current_offset = self.offset.get();
        let aligned_offset = align_up(current_offset, align);

        let new_offset = aligned_offset + size;
        if new_offset > self.capacity {
            panic!("Not enough capacity to allocate the value");
        }

        unsafe {
            let ptr = self.buffer.as_ptr().add(aligned_offset) as *mut T;
            ptr.write(value);
            self.offset.set(new_offset);
            &mut *ptr
        }
    }

    /// Resetea el arena para reusar la memoria
    pub fn reset(&self) {
        self.offset.set(0);
    }

    /// Retorna cuánta memoria se ha usado
    pub fn used(&self) -> usize {
        self.offset.get()
    }

    /// Retorna cuánta memoria queda disponible
    pub fn remaining(&self) -> usize {
        self.capacity - self.offset.get()
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.buffer.as_ptr(), self.layout);
        }
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_up() {
        assert_eq!(align_up(0, 8), 0);
        assert_eq!(align_up(1, 8), 8);
        assert_eq!(align_up(7, 8), 8);
        assert_eq!(align_up(8, 8), 8);
        assert_eq!(align_up(9, 8), 16);
    }

    #[test]
    fn test_basic_allocation() {
        let arena = Arena::new(1024).unwrap();
        let x = arena.alloc(42u32);
        assert_eq!(*x, 42);
    }

    // #[test]
    // #[ignore]
    // fn test_multiple_allocations() {
    //     let arena = Arena::new(1024);
    //     let x = arena.alloc(10u32);
    //     let y = arena.alloc(20u32);
    //     let z = arena.alloc(30u32);

    //     assert_eq!(*x, 10);
    //     assert_eq!(*y, 20);
    //     assert_eq!(*z, 30);
    // }
}

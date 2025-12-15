mod errors;

use std::alloc::dealloc;
use std::mem;
use std::{
    alloc::{Layout, alloc},
    cell::Cell,
    ptr::NonNull,
};

pub use crate::errors::ArenaError;

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

    pub fn alloc<T>(&self, value: T) -> Result<&mut T, ArenaError> {
        let size = mem::size_of::<T>();
        let align = mem::align_of::<T>();

        let current_offset = self.offset.get();
        let aligned_offset = align_up(current_offset, align);

        let new_offset = aligned_offset + size;
        if new_offset > self.capacity {
            return Err(ArenaError::NotEnoughCapacity);
        }

        unsafe {
            let ptr = self.buffer.as_ptr().add(aligned_offset) as *mut T;
            ptr.write(value);
            self.offset.set(new_offset);
            Ok(&mut *ptr)
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

use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

#[derive(Debug)]
enum AllocatorError {
    InvalidCapacity,
    AllocationFailed,
}

struct BumpAllocator {
    start: NonNull<u8>,
    layout: Layout,
    cursor: usize,
}

impl BumpAllocator {
    fn new(capacity: usize) -> Result<Self, AllocatorError> {
        if capacity == 0 {
            return Err(AllocatorError::InvalidCapacity);
        }

        let layout =
            Layout::from_size_align(capacity, 1).map_err(|_| AllocatorError::InvalidCapacity)?;

        let start =
            NonNull::new(unsafe { alloc(layout) }).ok_or(AllocatorError::AllocationFailed)?;

        Ok(Self {
            start,
            layout,
            cursor: 0,
        })
    }
}

impl Drop for BumpAllocator {
    fn drop(&mut self) {
        unsafe { dealloc(self.start.as_ptr(), self.layout) };
    }
}

fn main() {
    let allocator = BumpAllocator::new(1024).expect("failed to create allocator");
    println!(
        "Created a {}-byte allocator (cursor: {})",
        allocator.layout.size(),
        allocator.cursor
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_an_empty_allocator() {
        let allocator = BumpAllocator::new(64).unwrap();
        assert_eq!(allocator.layout.size(), 64);
        assert_eq!(allocator.cursor, 0);
    }

    #[test]
    fn rejects_invalid_capacity() {
        assert!(matches!(
            BumpAllocator::new(0),
            Err(AllocatorError::InvalidCapacity)
        ));
        assert!(matches!(
            BumpAllocator::new(usize::MAX),
            Err(AllocatorError::InvalidCapacity)
        ));
    }
}

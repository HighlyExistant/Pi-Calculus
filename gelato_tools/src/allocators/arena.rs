use std::{alloc::Layout, cell::{Cell, UnsafeCell}, ops::{Deref, DerefMut}, ptr::NonNull};

use crate::allocators::error::AllocError;

pub unsafe trait ArenaAllocator {
    fn scope(&self, size: usize) -> Result<ScopedArena, AllocError>;
    fn alloc_region(&self, size: usize) -> Result<NonNull<[u8]>, AllocError>;
    fn allocatable_bytes(&self) -> usize;
    fn clear(&self);
}

impl dyn ArenaAllocator {
    #[inline]
    pub fn alloc_slice<T>(&self, count: usize) -> Result<&mut [T], AllocError> {
        self.alloc_region(std::mem::size_of::<T>()*count)
            .map(|a|{
            unsafe { std::slice::from_raw_parts_mut(a.as_ptr().cast::<T>(), count) }
        })
    }
}

struct ScopedArenaInner {
    start: *mut u8,
    cursor: *mut u8,
    end: *mut u8,
}
pub struct ScopedArena {
    inner: UnsafeCell<ScopedArenaInner>
}

impl ScopedArena {
    unsafe fn from_raw(start: *mut u8, size: usize) -> Self {
        unsafe {
            Self { inner: UnsafeCell::new(ScopedArenaInner { start, cursor: start, end: start.add(size) }) }
        }
    }
    fn inner(&self) -> &mut ScopedArenaInner {
        unsafe { self.inner.get().as_mut().unwrap() }
    }
    fn cursor(&self) -> *mut u8 {
        self.inner().cursor
    }
    fn start(&self) -> *mut u8 {
        self.inner().start
    }
    fn end(&self) -> *mut u8 {
        self.inner().end
    }
    pub fn allocatable(&self, size: usize) -> bool {
        if (self.cursor() as usize + size) > self.end() as usize {
            println!("{}", size);
            println!("{}", self.start() as usize);
            println!("{}", self.cursor() as usize + size);
            println!("{}", self.end() as usize);
            false
        } else {
            true
        }
    }
    #[inline]
    pub fn alloc_slice<T>(&self, count: usize) -> Result<&mut [T], AllocError> {
        self.alloc_region(std::mem::size_of::<T>()*count)
            .map(|a|{
            unsafe { std::slice::from_raw_parts_mut(a.as_ptr().cast::<T>(), count) }
        })
    }
}

unsafe impl ArenaAllocator for ScopedArena {
    fn scope(&self, size: usize) -> Result<ScopedArena, AllocError> {
        if !self.allocatable(size) {
            return Err(AllocError::InsufficientSize);
        }
        unsafe {
            Ok(Self::from_raw(self.cursor(), size))
        }
    }
    fn alloc_region(&self, size: usize) -> Result<NonNull<[u8]>, AllocError> {
        if !self.allocatable(size) {
            return Err(AllocError::InsufficientSize);
        }
        unsafe {
            let x = std::slice::from_raw_parts_mut(self.cursor() as *mut u8, size);
            let alloc = NonNull::new_unchecked(x);
            self.inner().cursor = self.cursor().add(size); 
            Ok(alloc)
        }
    }
    fn allocatable_bytes(&self) -> usize {
        self.end() as usize - self.cursor() as usize
    }
    fn clear(&self) {
        self.inner().cursor = self.inner().start;
    }
}

pub struct MemoryArena {
    arena: ScopedArena,
}

impl MemoryArena {
    pub fn new(size: usize) -> Self {
        unsafe {
            let start = std::alloc::alloc(
                Layout::from_size_align(size, 1).unwrap()
            );
            Self { arena: ScopedArena::from_raw(start, size) }
        }
    }
    #[inline]
    pub fn alloc_slice<T>(&self, count: usize) -> Result<&mut [T], AllocError> {
        self.alloc_region(std::mem::size_of::<T>()*count)
            .map(|a|{
            unsafe { std::slice::from_raw_parts_mut(a.as_ptr().cast::<T>(), count) }
        })
    }
}

unsafe impl ArenaAllocator for MemoryArena {
    fn scope(&self, size: usize) -> Result<ScopedArena, AllocError> {
        self.arena.scope(size)
    }
    fn alloc_region<'scope>(&self, size: usize) -> Result<NonNull<[u8]>, AllocError> {
        self.arena.alloc_region(size)
    }
    fn allocatable_bytes(&self) -> usize {
        self.arena.allocatable_bytes()
    }
    fn clear(&self) {
        self.arena.clear();
    }
}

impl Drop for MemoryArena {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(
                self.arena.start().cast(), 
                Layout::from_size_align(
                self.arena.allocatable_bytes(), 1
                ).unwrap()
            );
        }
    }
}

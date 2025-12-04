use std::{alloc::Layout, any::type_name, cell::UnsafeCell, marker::PhantomData, ops::Div, ptr::NonNull, sync::{Arc, Mutex}};

use crate::allocators::{arena::ArenaAllocator, error::AllocError};


pub trait PoolAllocator {
    type Allocation;
    fn allocate(&self) -> Result<Self::Allocation, AllocError>;
    fn deallocate(&self, allocation: Self::Allocation) -> Result<(), AllocError>;
}

pub struct Pool<T> {
    ptr: *mut u8,
    capacity: usize,
    free: UnsafeCell<Vec<usize>>,
    marker_: PhantomData<T>,
}

impl<T> Pool<T> {
    pub unsafe fn from_raw(ptr: *mut u8, size: usize) -> Self {
        assert!(size % std::mem::size_of::<T>() == 0, "size must be aligned to {}", type_name::<T>());
        Self { ptr, capacity: size.div(std::mem::size_of::<T>()), free: UnsafeCell::new(vec![0]), marker_: PhantomData }
    }
    pub unsafe fn from_slice(slice: &mut [T]) -> Self {
        Self { ptr: slice.as_mut_ptr().cast(), capacity: slice.len(), free: UnsafeCell::new(vec![0]), marker_: PhantomData }
    }
    pub fn from_arena(arena: &dyn ArenaAllocator, layout: Layout) -> Result<Self, AllocError> {
        let ptr = arena.alloc_region(layout.size())?;
        Ok(Self { ptr: ptr.as_ptr() as _, capacity: layout.size(), free: UnsafeCell::new(vec![0]), marker_: PhantomData })
    }
    pub fn free(&self) -> &mut Vec<usize> {
        unsafe { self.free.get().as_mut().unwrap() }
    }
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    pub fn alloc_from_pool(&self, alloc: NonNull<T>) -> bool {
        let diff = self.ptr as usize + (self.capacity*std::mem::size_of::<T>());
        diff > alloc.as_ptr() as usize
    }
}

impl<T> PoolAllocator for Pool<T> {
    type Allocation = NonNull<T>;
    fn allocate(&self) -> Result<Self::Allocation, AllocError> {
        if self.free().len() == 0 {
            Err(AllocError::InsufficientSize)?
        }
        if let Some(alloc) = self.free().pop() {
            if alloc >= self.capacity {
                Err(AllocError::InsufficientSize)?
            }
            self.free().push(alloc+1);
            unsafe {
                Ok(NonNull::new(self.as_ptr().cast::<T>().add(alloc)).unwrap())
            }
        } else {
            Err(AllocError::InsufficientSize)?
        }
    }
    fn deallocate(&self, allocation: Self::Allocation) -> Result<(), AllocError> {
        let offset = (allocation.as_ptr() as usize - self.as_ptr() as usize).div(std::mem::size_of::<T>());
        self.free().push(offset);
        Ok(())
    }
}

pub struct MemoryPool<T> {
    pools: Arc<Mutex<Vec<Pool<T>>>>,
}

impl<T> MemoryPool<T> {
    pub fn new() -> Self {
        Self { pools: Arc::new(Mutex::new(vec![])) }
    }
}
impl<T> PoolAllocator for MemoryPool<T> {
    type Allocation = NonNull<T>;
    fn allocate(&self) -> Result<Self::Allocation, AllocError> {
        let mut pools = self.pools.lock().unwrap();
        for pool in pools.iter() {
            if let Ok(alloc) = pool.allocate() {
                return Ok(alloc);
            }
        }
        let alloc = unsafe { std::alloc::alloc(Layout::array::<T>(1024).unwrap()) };
        unsafe { pools.push(Pool::from_raw(alloc, Layout::array::<T>(1024).unwrap().size())) };
        pools.last().unwrap().allocate()
    }
    fn deallocate(&self, allocation: Self::Allocation) -> Result<(), AllocError> {
        let pools = self.pools.lock().unwrap();
        for pool in pools.iter() {
            if pool.alloc_from_pool(allocation) {
                pool.deallocate(allocation);
                return Ok(());
            }
        }
        Err(AllocError::WrongAlloc)
    }
}
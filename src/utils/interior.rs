#![allow(clippy::mut_from_ref)]
use std::cell::UnsafeCell;

//SAFETY: Interior assumes that T is Sync + Send, i.e. synchronization stuff will be done by cell's content, not by this pointer
pub struct Interior<T: ?Sized + Sync + Send> {
    cell: UnsafeCell<T>,
}

impl<T> Interior<T>
where
    T: Sized + Sync + Send,
{
    pub fn new(data: T) -> Interior<T> {
        Interior {
            cell: UnsafeCell::new(data),
        }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.cell.get() }
    }
}
unsafe impl<T: Sized + Sync + Send> Sync for Interior<T> {}
unsafe impl<T: Sized + Sync + Send> Send for Interior<T> {}

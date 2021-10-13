

use core::cell::UnsafeCell;

pub mod interface {

    // Any object implementing this trait guarantees exclusive access to the
    // data wrapped within the mutex for the duration of the provided closure. 
    pub trait Mutex {
        // The type of the data that is wrapped by this mutex.
        type Data;

        // locks the mutext and grants the closure temporary mutable access
        fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
    }
}


/// a pseudo-lock for teaching purposes.

pub struct NullLock<T> where T: ?Sized,
{
    data: UnsafeCell<T>
}


unsafe impl<T> Send for NullLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for NullLock<T> where T: ?Sized + Sync {}

impl<T> NullLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}


// os interface

impl<T> interface::Mutex for NullLock<T> {
    type Data = T;

    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        // in a real lock, there would be code encapsulating this line  that ensure that this
        // mutable reference will ever only be given out once at a time.
        let data = unsafe { &mut *self.data.get() };

        f(data)
    }
}

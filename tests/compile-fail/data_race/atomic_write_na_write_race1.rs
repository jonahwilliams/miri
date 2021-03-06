// ignore-windows: Concurrency on Windows is not supported yet.
#![feature(core_intrinsics)]

use std::thread::spawn;
use std::sync::atomic::AtomicUsize;
use std::intrinsics::atomic_store;

#[derive(Copy, Clone)]
struct EvilSend<T>(pub T);

unsafe impl<T> Send for EvilSend<T> {}
unsafe impl<T> Sync for EvilSend<T> {}

pub fn main() {
    let mut a = AtomicUsize::new(0);
    let b = &mut a as *mut AtomicUsize;
    let c = EvilSend(b);
    unsafe {
        let j1 = spawn(move || {
            *(c.0 as *mut usize) = 32;
        });

        let j2 = spawn(move || {
            //Equivalent to: (&*c.0).store(64, Ordering::SeqCst)
            atomic_store(c.0 as *mut usize, 64); //~ ERROR Data race
        });

        j1.join().unwrap();
        j2.join().unwrap();
    }
}

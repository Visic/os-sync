//!Semaphore primitive

#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
mod posix;
#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
pub use posix::Sem;

#[cfg(windows)]
mod win32;
#[cfg(windows)]
pub use win32::Sem;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod mac;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use mac::Sem;

///Describes Semaphore interface
///
///This primitive provides access to single integer that can be decremented using post
///and incremented using wait
pub trait Semaphore: Sized {
    ///Creates new instance, returning None on inability to do so.
    ///
    ///`init` is initial value for the semaphore
    fn new(init: u32) -> Option<Self>;

    ///Decrements self, returning immediately if it was signaled.
    ///
    ///Otherwise awaits for `post`
    fn wait(&self);

    ///Attempts to decrement self, returning whether self was signaled or not.
    ///
    ///Returns `true` if self was signaled
    ///
    ///Returns `false` otherwise
    fn try_wait(&self) -> bool;

    ///Increments self, returns Some with result of incremention
    ///Otherwise, returns None, meaning it is failed due to underlying OS error.
    ///
    ///When self becomes greater than zero, waiter shall be woken and result is `true`
    fn post(&self) -> Option<bool>;
}

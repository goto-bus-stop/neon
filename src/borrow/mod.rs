//! Types and traits for obtaining temporary access to the internals of JavaScript values.

pub(crate) mod internal;

use std::ops::{Deref, DerefMut, Drop};
use std::fmt;
use std::os::raw::c_void;

use context::{Context, Lock};
use self::internal::Pointer;

/// A trait for JS values whose internal contents can be borrowed immutably by Rust while the JS engine is locked.
pub trait Borrow: Sized {

    /// The type of the value's internal contents.
    type Target: Pointer;

    /// Borrow the contents of this value immutably.
    ///
    /// If there is already an outstanding mutable loan for this value, this method panics.
    fn borrow<'c, 'a: 'c, C: Context<'c>>(self, lock: &'a Lock<'c, 'a, C>) -> Ref<'c, 'a, Self::Target, C> {
        match self.try_borrow(lock) {
            Ok(r) => r,
            Err(e) => panic!("{}", e)
        }
    }

    /// Borrow the contents of this value immutably.
    ///
    /// If there is already an outstanding mutable loan for this value, this method fails with a `LoanError`.
    fn try_borrow<'c, 'a: 'c, C: Context<'c>>(self, lock: &'a Lock<'c, 'a, C>) -> Result<Ref<'c, 'a, Self::Target, C>, LoanError>;

}

/// A trait for JS values whose internal contents can be borrowed mutably by Rust while the JS engine is locked.
pub trait BorrowMut: Borrow {

    /// Borrow the contents of this value mutably.
    ///
    /// If there is already an outstanding loan for this value, this method panics.
    fn borrow_mut<'c, 'a: 'c, C:Context<'c>>(self, lock: &'a Lock<'c, 'a, C>) -> RefMut<'c, 'a, Self::Target, C> {
        match self.try_borrow_mut(lock) {
            Ok(r) => r,
            Err(e) => panic!("{}", e)
        }
    }

    /// Borrow the contents of this value mutably.
    ///
    /// If there is already an outstanding loan for this value, this method panics.
    fn try_borrow_mut<'c, 'a: 'c, C:Context<'c>>(self, lock: &'a Lock<'c, 'a, C>) -> Result<RefMut<'c, 'a, Self::Target, C>, LoanError>;

}

/// An error produced by a failed loan in the `Borrow` or `BorrowMut` traits.
pub enum LoanError {

    /// Indicates that there is already an outstanding mutable loan for the object at this address.
    Mutating(*const c_void),

    /// Indicates that there is already an outstanding immutable loan for the object at this address.
    Frozen(*const c_void)

}

impl fmt::Display for LoanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoanError::Mutating(p) => {
                write!(f, "outstanding mutable loan exists for object at {:?}", p)
            }
            LoanError::Frozen(p) => {
                write!(f, "object at {:?} is frozen", p)
            }
        }
    }
}

/// An immutable reference to the contents of a borrowed JS value.
pub struct Ref<'c, 'a: 'c, T: Pointer, C: Context<'c>> {
    pointer: T,
    lock: &'a Lock<'c, 'a, C>
}

impl<'c, 'a: 'c, T: Pointer, C: Context<'c>> Ref<'c, 'a, T, C> {
    pub(crate) unsafe fn new(lock: &'a Lock<'c, 'a, C>, pointer: T) -> Result<Self, LoanError> {
        let mut ledger = lock.ledger.borrow_mut();
        ledger.try_borrow(pointer.as_ptr())?;
        Ok(Ref { pointer, lock })
    }
}

impl<'c, 'a: 'c, T: Pointer, C: Context<'c>> Drop for Ref<'c, 'a, T, C> {
    fn drop(&mut self) {
        let mut ledger = self.lock.ledger.borrow_mut();
        ledger.settle(unsafe { self.pointer.as_ptr() });
    }
}

impl<'c, 'a: 'c, T: Pointer, C: Context<'c>> Deref for Ref<'c, 'a, T, C> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.pointer
    }
}

/// A mutable reference to the contents of a borrowed JS value.
pub struct RefMut<'c, 'a: 'c, T: Pointer, C: Context<'c>> {
    pointer: T,
    lock: &'a Lock<'c, 'a, C>,
}

impl<'c, 'a: 'c, T: Pointer, C: Context<'c>> RefMut<'c, 'a, T, C> {
    pub(crate) unsafe fn new(lock: &'a Lock<'c, 'a, C>, mut pointer: T) -> Result<Self, LoanError> {
        let mut ledger = lock.ledger.borrow_mut();
        ledger.try_borrow_mut(pointer.as_mut())?;
        Ok(RefMut { pointer, lock })
    }
}

impl<'c, 'a: 'c, T: Pointer, C: Context<'c>> Drop for RefMut<'c, 'a, T, C> {
    fn drop(&mut self) {
        let mut ledger = self.lock.ledger.borrow_mut();
        ledger.settle_mut(unsafe { self.pointer.as_mut() });
    }
}

impl<'c, 'a: 'c, T: Pointer, C: Context<'c>> Deref for RefMut<'c, 'a, T, C> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.pointer
    }
}

impl<'c, 'a: 'c, T: Pointer, C: Context<'c>> DerefMut for RefMut<'c, 'a, T, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pointer
    }
}

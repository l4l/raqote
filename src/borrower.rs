use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

pub enum Borrower<'a, T: ?Sized + ToOwned> {
    Owned(<T as ToOwned>::Owned),
    BorrowedMut(&'a mut T),
}

impl<'a, T: ?Sized + ToOwned> Borrower<'a, T> {
    pub fn owned(t: <T as ToOwned>::Owned) -> Self {
        Self::Owned(t)
    }

    pub fn borrowed(t: &'a mut T) -> Self {
        Self::BorrowedMut(t)
    }

    pub fn into_owned(self) -> <T as ToOwned>::Owned {
        match self {
            Borrower::Owned(x) => x,
            Borrower::BorrowedMut(x) => x.to_owned(),
        }
    }
}

impl<'a, T: ?Sized + ToOwned> Deref for Borrower<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Borrower::Owned(ref x) => x.borrow(),
            Borrower::BorrowedMut(ref x) => x,
        }
    }
}

impl<'a, T: ?Sized + ToOwned> DerefMut for Borrower<'a, T>
where
    <T as ToOwned>::Owned: BorrowMut<T>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Borrower::Owned(ref mut x) => x.borrow_mut(),
            Borrower::BorrowedMut(ref mut x) => x,
        }
    }
}

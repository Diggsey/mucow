//! A module for working with mutably borrowed data.

use std::fmt;
use std::borrow::{Borrow, BorrowMut, Cow};
use std::ops::{Deref, DerefMut};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use self::MuCow::*;


impl<'a, B: ?Sized> Borrow<B> for MuCow<'a, B>
    where B: ToOwned,
          B::Owned: 'a
{
    fn borrow(&self) -> &B {
        &**self
    }
}

impl<'a, B: ?Sized> BorrowMut<B> for MuCow<'a, B>
    where B: ToOwned,
          B::Owned: 'a + BorrowMut<B>
{
    fn borrow_mut(&mut self) -> &mut B {
        &mut **self
    }
}

/// A clone-on-consume smart pointer.
pub enum MuCow<'a, B: ?Sized + 'a>
    where B: ToOwned
{
    /// Borrowed data.
    Borrowed(&'a mut B),

    /// Owned data.
    Owned(<B as ToOwned>::Owned),
}

impl<'a, B: ?Sized> Into<Cow<'a, B>> for MuCow<'a, B> where B: ToOwned {
    fn into(self) -> Cow<'a, B> {
        match self {
            Borrowed(borrowed) => Cow::Borrowed(borrowed),
            Owned(owned) => Cow::Owned(owned)
        }
    }
}

impl<'a, B: ?Sized> Clone for MuCow<'a, B> where B: ToOwned {
    fn clone(&self) -> MuCow<'a, B> {
        Owned((&**self).to_owned())
    }
}

impl<'a, B: ?Sized> MuCow<'a, B> where B: ToOwned {
    /// Extracts the owned data.
    ///
    /// Clones the data if it is not already owned.
    pub fn into_owned(self) -> <B as ToOwned>::Owned {
        match self {
            Borrowed(borrowed) => borrowed.to_owned(),
            Owned(owned) => owned,
        }
    }
}

impl<'a, B: ?Sized> Deref for MuCow<'a, B> where B: ToOwned {
    type Target = B;

    fn deref(&self) -> &B {
        match *self {
            Borrowed(ref borrowed) => borrowed,
            Owned(ref owned) => owned.borrow(),
        }
    }
}

impl<'a, B: ?Sized> DerefMut for MuCow<'a, B> where B: ToOwned, B::Owned: BorrowMut<B> {
    fn deref_mut(&mut self) -> &mut B {
        match *self {
            Borrowed(ref mut borrowed) => borrowed,
            Owned(ref mut owned) => owned.borrow_mut(),
        }
    }
}

impl<'a, B: ?Sized> Eq for MuCow<'a, B> where B: Eq + ToOwned {}

impl<'a, B: ?Sized> Ord for MuCow<'a, B> where B: Ord + ToOwned {
    #[inline]
    fn cmp(&self, other: &MuCow<'a, B>) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl<'a, 'b, B: ?Sized, C: ?Sized> PartialEq<MuCow<'b, C>> for MuCow<'a, B>
    where B: PartialEq<C> + ToOwned,
          C: ToOwned
{
    fn eq(&self, other: &MuCow<'b, C>) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}

impl<'a, B: ?Sized> PartialOrd for MuCow<'a, B> where B: PartialOrd + ToOwned {
    #[inline]
    fn partial_cmp(&self, other: &MuCow<'a, B>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}

impl<'a, B: ?Sized> fmt::Debug for MuCow<'a, B>
    where B: fmt::Debug + ToOwned,
          <B as ToOwned>::Owned: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Borrowed(ref b) => fmt::Debug::fmt(b, f),
            Owned(ref o) => fmt::Debug::fmt(o, f),
        }
    }
}

impl<'a, B: ?Sized> fmt::Display for MuCow<'a, B>
    where B: fmt::Display + ToOwned,
          <B as ToOwned>::Owned: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Borrowed(ref b) => fmt::Display::fmt(b, f),
            Owned(ref o) => fmt::Display::fmt(o, f),
        }
    }
}

impl<'a, B: ?Sized> Default for MuCow<'a, B>
    where B: ToOwned,
          <B as ToOwned>::Owned: Default
{
    fn default() -> MuCow<'a, B> {
        Owned(<B as ToOwned>::Owned::default())
    }
}

impl<'a, B: ?Sized> Hash for MuCow<'a, B> where B: Hash + ToOwned {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}

impl<'a, T: ?Sized + ToOwned> AsRef<T> for MuCow<'a, T> {
    fn as_ref(&self) -> &T {
        self
    }
}

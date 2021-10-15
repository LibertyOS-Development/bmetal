// BMETAL: A simple crate to provide the basic abstractions that one would need for developing for bare-metal targets.
#![no_std]

use core::cell::UnsafeCell;
use core::marker::PhantomData;

// CritSect: This is a token that, when used, indicates that there is code
// being executed, by the current code, within a critical section. This token
// is designed to let the system know about the aforementioned situation, so
// that nothing interrupts the code that is being executed.

#[derive(Clone, Copy, Debug)]
pub struct CritSect<'cs>
{
	_0: PhantomData<&'cs ()>,
}

impl<'cs> CritSect<'cs>
{
	#[inline(always)]
	pub unsafe fn new() -> Self
	{
		CritSect
		{
			_0: PhantomData
		}
	}
}

#[derive(Debug)]
pub struct Mutex<T>
{
	inner: UnsafeCell<T>,
}
impl<T> Mutex<T>
{
	pub const fn new(val: T) -> Self
	{
		Mutex
		{
			inner: UnsafeCell::new(val),
		}
	}
	pub fn getmut(&mut self) -> &mut T
	{
		unsafe
		{
			&mut *self.inner.get()
		}
	}
	pub fn intoinner(self) -> T
	{
		self.inner.into_inner()
	}
	pub fn borrow<'cs>(&'cs self, _cs: CritSect<'cs>) -> &'cs T
	{
		unsafe
		{
			&*self.inner.get()
		}
	}
}
unsafe impl<T> Sync for Mutex<T> where T: Send {}

#[allow(dead_code)]
const GH6: () = ();

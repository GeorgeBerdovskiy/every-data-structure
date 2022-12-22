use std::ptr::NonNull;
use std::alloc::{self, Layout};

pub struct Vector<T> {
	// Points to first element in the vector (NonNull pointer)
	pointer: NonNull<T>,

	// Number of elements inside the vector
	length: usize,

	// Capacity currently allocated for the vector
	capacity: usize
}

impl<T> Vector<T> {
	pub fn new() -> Self {
		Self {
			pointer: NonNull::dangling(),
			length: 0,
			capacity: 0
		}
	}

	pub fn push(&mut self) {
		if self.capacity == 0 {
			// Allocate heap memory for our vector
			let layout = alloc::Layout::array::<T>(4).expect("Could not allocate.");
			
			// SAFETY NOTE - Layout is hardcoded to be 4 * size_of<T>
			let pointer = unsafe { alloc::alloc(layout) } as *mut T;
		}
	}

	pub fn capacity(&self) -> usize {
		self.capacity
	}

	pub fn length(&self) -> usize {
		self.length
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_works() {
        let vec = Vector::<usize>::new();

		assert_eq!(vec.capacity(), 0);
		assert_eq!(vec.length(), 0);
	}
}

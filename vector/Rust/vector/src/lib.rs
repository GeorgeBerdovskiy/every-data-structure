use std::ptr::NonNull;
use std::alloc::{self};

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

	pub fn push(&mut self, item: T) {
		// Needed to meet safety invariant (don't allow zero sized types such as 'unit')
		assert_ne!(std::mem::size_of::<T>(), 0, "Type cannot be zero sized.");

		// Allocate memory for vector
		if self.capacity == 0 {
			// Allocate heap memory for our vector
			let layout = alloc::Layout::array::<T>(4).expect("Could not allocate.");
			
			// SAFETY: Layout is hardcoded to be 4 * size_of<T>
			let pointer = unsafe { alloc::alloc(layout) } as *mut T;

			// Shadowing 'pointer'
			let pointer = NonNull::new(pointer).expect("Could not allocate memory.");
		
			// SAFETY: Pointer is non-null and we have just allocated enough space for this item
			// The memory previously at 'pointer' is NOT read!
			unsafe { pointer.as_ptr().write(item) };

			self.pointer = pointer;
			self.capacity = 4;
			self.length = 1;
		} else if self.length < self.capacity {
			// Not sure if this is strictly necessary, but we'll leave it for now - definitely safe
			let offset = self.length
				.checked_mul(std::mem::size_of::<T>())
				.expect("Cannot reach memory location.");

			assert!(offset < isize::MAX as usize, "Wrapped isize.");

			// Offset cannot wrap around and pointer is pointing to valid memory
			// And writing to an offset at self.length is valid

			unsafe {
				// 'Skip ahead' to point to the first uninitialized item in allocated memory
				self.pointer.as_ptr().add(self.length).write(item)
			}

			self.length += 1;
		} else {
			debug_assert!(self.length == self.capacity);

			// Need to increase capacity before adding another element
			// NOTE - Not all of the expects may be necessary, look into which ones can be safely removed
			let new_capacity = self.capacity.checked_mul(2).expect("Capacity wrapped.");

			let align = std::mem::align_of::<T>();
			let size = std::mem::size_of::<T>() * self.capacity;

			size.checked_add(size % align).expect("Cannot allocate - wrapped.");

			let pointer = unsafe {
				let layout = alloc::Layout::from_size_align_unchecked(size, align);
				
				// Should be in bytes
				let new_size = std::mem::size_of::<T>() * new_capacity; 

				let pointer = alloc::realloc(self.pointer.as_ptr() as *mut u8, layout, new_size);
				let pointer = NonNull::new(pointer as *mut T).expect("Could not reallocate.");
			
				pointer.as_ptr().add(self.length).write(item);
				pointer
			};

			self.pointer = pointer;
			self.length += 1;
			self.capacity = new_capacity;
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
        let mut vec = Vector::<usize>::new();

		vec.push(1usize);
		vec.push(2);
		vec.push(3);
		vec.push(4);
		vec.push(5);

		assert_eq!(vec.capacity(), 8);
		assert_eq!(vec.length(), 5);
	}
}

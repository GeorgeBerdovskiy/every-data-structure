struct SimpleHashTableItem {
	key: String,
	value: i32
}

struct SimpleHashTable {
    length: i32 // Number of key/value pairs
}

impl SimpleHashTable {
	// Initialize a new SimpleHashTable
	pub fn new(length: i32) -> Self {
		Self {
			length
		}
	}

	// Return number of key/value pairs stored inside the HashTable
    pub fn length(&self) -> i32 {
        self.length
    }
}

fn main() {
	let hash_table = SimpleHashTable::new(2);
    println!("Hello! The SimpleHashTable has been initialized and has a length of {}.", hash_table.length());
}

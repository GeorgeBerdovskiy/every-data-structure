#include <stdlib.h>

typedef struct {
    int * bucket;
    int current_size;
} HashTable;

// TODO - Initialize with smaller, more appropriate size
HashTable * hashTableCreate() {
    HashTable * hashtable = (HashTable*) malloc(sizeof(int) * 1000001);
    
    hashtable -> bucket = (int*) malloc(sizeof(int) * 1000001);
    hashtable -> current_size = 1000001;
    
    for (int i = 0; i < 1000001; i++) {
        hashtable -> bucket[i] = -1;
    }
    
    return hashtable;
}

void resize_map(HashTable* obj, int maximum_key)  {
    // TODO - Implement resizing  
}

void hashTablePut(HashTable* obj, int key, int value) {
    int hashed_index = key;
    obj -> bucket[hashed_index] = value;
}

int hashTableGet(HashTable* obj, int key) {
    return obj -> bucket[key];
}

void hashTableRemove(HashTable* obj, int key) {
  obj -> bucket[key] = -1;
}

void hashTableFree(HashTable* obj) {
    // TODO - Clean data first (maybe)
    free(obj);
}

int main() {
    // TODO - Initialize and perform operations on hash table
    return 0;
}

# Rust implementation of Database datastuctures outlined in "Designing Data-Intensive Applications, O'Reilly 2017"
Chapter 3: Storage and Retrievial

SimpleDB:   Appends strings to a file seperated by newlines and then searches in reverse when reading
 
HashIdxDB:  Seperates data into segments, each with an associated in-memory hash table look-up
 
LSMTreeDb:  Implements a LSM Tree with in-memory memtable and background workers
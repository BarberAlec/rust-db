mod simple_db;
mod hash_index_db;

/// Rust implementation of Database datastuctures outlined in "Designing Data-Intensive Applications, O'Reilly 2017"
/// Chapter 3: Storage and Retrievial
///
/// SimpleDB:   Appends strings to a file seperated by newlines and then searches in reverse when reading
/// 
/// HashIdxDB:  Seperates data into segments, each with an associated in-memory hash table look-up
/// 
/// LSMTreeDb:  Implements a LSM Tree with in-memory memtable and background workers
fn main() {
    let mut mydb = hash_index_db::HashIdxDB::new();
    mydb.write("first_key", "{\"hello\": [1, 2, 3, 4]}").expect("warning");
    mydb.write("alec", "new_value").expect("warning");
    mydb.write("alec", "very_new value here").expect("warning");


    // let msg = mydb.read("first_key").expect("somethin went wrong");
    // println!("{}", msg.to_string());

    // let msg = mydb.read("alec").expect("somethin went wrong");
    // println!("{}", msg.to_string());

}

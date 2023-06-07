mod simple_db;


fn main() {
    let mut mydb = simple_db::SimpleDB::new();
    mydb.write("first_key", "{\"hello\": [1, 2, 3, 4]}").expect("warning");
    mydb.write("alec", "new_value").expect("warning");
    mydb.write("alec", "very_new value here").expect("warning");


    let msg = mydb.read("first_key").expect("somethin went wrong");
    println!("{}", msg.to_string());

    let msg = mydb.read("alec").expect("somethin went wrong");
    println!("{}", msg.to_string());

}

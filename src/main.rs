#[allow(unused_imports)]
mod vector;

use vector::VectorDB;

fn main() {
    let mut db = VectorDB::new();

    // for i in 0..1000 {
    //     db.put(i, i * 2);
    // }

    db.add("vector1".to_string(), vec![1.0, 2.0, 3.0]);

    println!("{:?}", db.get("vector1"));

    db.update("vector1", vec![4.0, 5.0, 6.0]);

    println!("{:?}", db.get("vector1"));

    db.remove("vector1");

    println!("{:?}", db.get("vector1"));

    println!("Hello, Vector!");
}

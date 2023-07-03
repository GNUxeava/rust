fn main () {
    let mut vector: Vec<i32> = Vec::new();
    for i in 0..10 {
        vector.push(i);
    }

    for i in 0..10 {
        println!("Element => {}: Value => {}", &i, &vector[i]);
    }
}

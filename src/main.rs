fn main() {
    let source_string = "foobar";
    let my_vec: Vec<char> = source_string.chars().collect();
    println!("my_vec[0]: {}", my_vec[0]);
    /* for i in my_vec {
        println!("{}", i);
    } */
    let mut index = 0;
    let mut string = String::new();

    while index < my_vec.len() {
        println!("{} < indexing", my_vec[index]);
        if my_vec[index] == my_vec[index + 1] {
            string.push(my_vec[index]);
        }
        if index != 0 {
            if my_vec[index] == my_vec[index - 1] {
                string.push(my_vec[index]);
            }
        }
        index += 1;
        println!("{}", string);
    }
    println!("{}", string);
}

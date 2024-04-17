fn main() {
    // Creating new vector.
    let v: Vec<i32> = Vec::new();
    // Using vec! macro.
    let v = vec![1, 2, 3];
    // Updating.
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading use index.
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // Reading use get method, which returns Option<>.
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element in array."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // This returns None without panic.
    let does_not_exist = v.get(100);
    // This will cause panic, i.e. stops executing.
    // let does_not_exist = &v[100];

    // Borrowing and referencing.
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    // Adding a new element to the end will cause new memort allocation and copying old values to new
    // space.
    // v.push(6);

    println!("The first element is: {first}");

    // Iterating.
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // To change the value that the mutable reference refers to we have to use dereference operator
    // * to get the value in i before += operator.
    let mut v = vec![100, 32, 57];
    println!("Before changing v");
    for i in &v {
        println!("{i}");
    }

    println!("Dereferencing each element");
    for i in &mut v {
        *i += 50;
    }

    println!("After changing v");
    for i in &v {
        println!("{i}");
    }
}

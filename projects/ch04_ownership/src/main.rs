// Rules of Refs:
// - Can have either one mutable ref or any number of immutable refs
// - Refs must always be valid

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // Fails cuz we've transferred ownership to s2
    println!("{}, world!", s2);

    let s2clone = s2.clone();
    println!("s2 = {s2}, s2clone = {s2clone}");

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    function_call();

    let s3 = gives_ownership();
    println!("s3 = {}", s3);

    let s3 = takes_and_gives_back(s3);
    println!("s3 = {s3}");

    let (s4, len) = calculate_length_move(s3);
    println!("s4={s4}, len={len}");

    println!("s4={s4}, len={}", calculate_length_borrow(&s4));

    let mut s5 = s4;
    println!("s5_before={s5}");
    change(&mut s5);
    println!("s5_after={s5}");

    // Can't have two mutable references at the same time (in the same scope?)
    // to prevent trying to mutate the same data
    //
    // ```
    // let r1 = &mut s5;
    // let r2 = &mut s5;
    // println!("{r1}, {r2}");
    // ```

    {
        let _r1 = &mut s5;
    }
    let r2 = &mut s5;
    println!("r2={r2}");

    // Can't have both mutable and immutable borrows at the same time. We don't
    // have an immutable ref to change while it's alive.
    //
    // ```
    // let r1 = &s5; // Immutable ref, okay
    // let r2 = &s5; // Immutable ref, also fine
    // let r3 = &mut s5; // Oh no! Trying to create a mutable ref while immutable refs are valid won't work.
    // println!("r1={r1}, r2={r2}, r3={r3}");
    // ```

    let r1 = &s5; // Immutable ref, okay
    let r2 = &s5; // Immutable ref, also fine
    println!("r1={r1}, r2={r2}"); // Last usage of immutable refs
    let r3 = &mut s5; // Okay cuz no usages of immutable refs at this point
    println!("r3={r3}");

    dangling_ref();

    let fw_last_index = first_word_noslice(&s5);
    println!("fw_last_index={fw_last_index}");
    println!("first_word(&s5)={}", first_word(&s5));

    // Slices are an immutable borrow of a collection. So they prevent mutations
    // to the collection while they (the immutable borrows) are valid.
    // ```
    // let word = first_word(&s5);
    // s5.clear(); // Errors cuz it tries to take a mutable ref while `word` is an immutable ref
    // println!("word={word}");
    // ```
}

fn function_call() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("s = {}", s); // Fails cuz we've transferred ownership to takes_ownership

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("some_string = {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer = {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length_move(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_borrow(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn dangling_ref() {
    // let ref_to_nothing = dangle();
    let _s = no_dangle();
}

// fn dangle() -> &String {
//     // Returns a reference to a String, i.e. attempting to let the caller "borrow" a value owned by this function scope
//     let s = String::from("Hello"); // s is a new String
//     &s // Return a reference to String, s.
//        // However, s goes out of scope and is dropped, so it's memory goes away!
//        // So this won't work
// }

fn no_dangle() -> String {
    // Here, we return the string itself and therefore ownership of the string
    let s = String::from("hello");
    s
}

fn first_word_noslice(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

// Using &str as a parameter to work on string literals, &String and string slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

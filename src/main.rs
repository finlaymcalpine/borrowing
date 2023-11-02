fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // passing not s1, but a reference to s1 to the function

    println!("The length of '{}' is {}.", s1, len);

    // this function, which tries to make a change to the value it is pointing to, will compiler error
    //change(&s1);

    //mutable references
    let mut s2 = String::from("apples");

    mutable_refs(&mut s2);

    println!("{s2}");

}

fn calculate_length(s: &String) -> usize {
    // need to specify that the argument type is a reference to a String, not a String
    s.len()
} // when s goes out of scope here, what it is referring to (s1) is not dropped because s does not have ownership: it's a reference

// When functions have references as parameters instead of the actual values,
// we won’t need to return the values in order to give back ownership, because we never had ownership.

// BORROWING is the act= of creating a reference. If we try to modify something we've borrowed, it will fail.
// if we try to call this change function on &s1, we're told that parameter is a reference and is not mutable.
// References, as with variables, are immutable by default.

//fn change(s: &String) {
//    s.push_str(", world!");
//}

// we can change a value being referred to if we define it as a mutable reference as below. the restriction is that only one mutable 
// reference can be made to a value at a time. The base value does need to be mutable.

fn mutable_refs(s: &mut String) {
    s.push_str(" & bananas");
}

// We don't have dangling pointers in Rust, because the compiler will only allow references within the same scope as the data they refer to
// We can't, for example, return a reference to a value from a function, if the value was created inside the function. In that case,
// the value is dropped when the function exits. So we could only return the value itself out of the function if created within. 
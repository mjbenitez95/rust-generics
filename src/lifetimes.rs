pub fn good_lifetime() {
    {
        let x = 5;
        let r = &x;
        println!("r: {}", r);
    }
}

/*
    lifetime annotation syntax:
        &i32        // a reference
        &'a i32     // a reference with an explicit lifetime
        &'a mut i32 // a mutable reference with an explicit lifetime
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn main() {
    basic_lifetimes();
    acceptable_overlapping_lifetimes();
    lifetimed_struct();
}

fn basic_lifetimes() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn acceptable_overlapping_lifetimes() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn unacceptable_lifetime() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // this line would fail because result only lives as long as string2
    // println!("The longest string is {}", result);
}

// this annotation means an instance of this struct can't outlive what it's referencing
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetimed_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

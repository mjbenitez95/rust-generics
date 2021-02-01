use std::fmt::Display;

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

/*
    note that these two implemented functions help demonstrate the elision rules
    by which the compiler infers a lifetime for references as part of methods:
        1) all inputs get a unique lifetime
        2) if there's only one input, the output shares its lifetime
        3) if one of the inputs is &self, the output shares the lifetime of the object
*/
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn lifetimed_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!(
        "Result of i.level(): {} and i.announce_and_return_part(): {}",
        i.level(),
        i.announce_and_return_part("Our book begins with...")
    );
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn absolute_longest() {
    let s1 = String::from("some string");
    let s2 = String::from("some other really, really long string");
    println!(
        "  {}!",
        longest_with_an_announcement(&s1, &s2, "The absolute_longest() string is: ")
    );
}

pub fn main() {
    basic_lifetimes();
    acceptable_overlapping_lifetimes();
    lifetimed_struct();
    absolute_longest();
}

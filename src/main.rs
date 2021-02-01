fn main() {
    extract_function();
    generic_data_types();
    generic_struct();
}

fn extract_function() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}!", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}!", result);
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generic_data_types() {
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let char_list = vec!['y', 'm', 'a', 'c', 'q', 'e'];

    // let largest_number = largest_generic(&number_list);
    // let largest_char = largest_generic(&char_list);

    // println!("The largest number is {}!", largest_number);
    // println!("The largest char is {}!", largest_char);
}

// fn largest_generic<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

struct Point<T> {
    x: T,
    y: T,
}

fn generic_struct() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };
}

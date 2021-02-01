mod lib;

fn main() {
    extract_function();
    generic_data_types();
    generic_struct();
    println!();

    lib::traits();
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

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn generic_struct() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let will_work = Point { x: 5, y: 4.0 };
    println!("The x component of {:?} is {}!", will_work, will_work.x());
    println!(
        "The distance from origin of {:?} is {}!",
        float,
        float.distance_from_origin()
    );

    let p1 = Point { x: "Hello", y: 'c' };
    let p2 = Point { x: 5, y: 10.4 };
    let p3 = p1.mixup(p2);

    println!("p3 is {:?}!", p3);
}

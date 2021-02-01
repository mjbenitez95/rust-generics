use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<t> {
    fn new(x: T,y: T) -> Self{
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member in {:?} is x = {}!", self, self.x);
        } else {
            println!("The largest member in {:?} is y = {}!", self, self.y);
        }
    }
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Point<T> {
    x: T,
    y: T,
}

// Here impl<T> is the type of the struct like let p = Point { x: 5, y: 10 };
// then T is the int32
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// This code means the type Point<f32> will have a distance_from_origin method; other instances of Point<T>
// where T is not of type f32 will not have this method defined. The method measures how far our point is
// from the point at coordinates (0.0, 0.0) and uses mathematical operations that are available only for floating point types.

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// Here impl<T> is the type of the struct like let p = Point { x: 5, y: 10 };
// then T is the int32 but when we do p.mixup(p2) then mixup<X2,Y2> X2,Y2 are the type of the parameter
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

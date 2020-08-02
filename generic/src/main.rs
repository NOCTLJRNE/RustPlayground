struct Point<T, U> {
    x: T,
    y: U,
}
impl<T: Copy, U> Point<T, U> {
    fn mixup<V, W: Copy>(&self, other: &Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    let p1 = Point { x: 5, y: 3.14 };
    let p2 = Point { x: "Hi", y: 'Y' };
    let p3 = p1.mixup(&p2);
    let p4 = p2.mixup(&p1);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!("p4.x = {}, p4.y = {}", p4.x, p4.y);
}

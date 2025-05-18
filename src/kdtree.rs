struct KdTree{ 
    value: Point,
    left: Option<Box<KdTree>>,
    right: Option<Box<KdTree>>,
}

enum SplitAxis{
    X,
    Y
}
struct Point {
    x: i32,
    y: i32,
}

impl Point{
    fn new(x: i32, y: i32) -> Point{
        Point{x: x, y: y}
    }
    
}

impl KdTree{
    fn new(value:  Point) -> KdTree{
        KdTree{value: value, left: None, right: None}
    }
    
}
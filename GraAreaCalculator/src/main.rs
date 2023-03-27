fn main() {
    let Square = Square{ side: 1.0 };
    let Rectangle = Rectangle{ base: 2.0, height: 1.0 };
    let Circle = Circle{ radius: 1.0 }; 
    let Triangle = Triangle{ base: 2.0, height: 1.0 };

    print_res(Square);
    print_res(Rectangle);
    print_res(Circle);
    print_res(Triangle);
}
trait Area {
    fn calculate(&self) -> f64;
}

//定义图形的结构体：
//定义正方形结构体；
struct Square {
    side: f64,
}

//定义矩形结构体：
struct Rectangle {
    base: f64,
    height: f64,
}

//定义圆形结构体：
struct Circle {
    radius: f64,
}

//定义三角形结构体：
struct Triangle{
    base: f64,
    height: f64,
}

//定义图形计算函数：
//定义正方形计算函数：
impl Area for Square {
    fn calculate(&self) -> f64 {
        let area = self.side * self.side;
        return area;
    }
}

//定义矩形计算函数：
impl Area for Rectangle {
    fn calculate(&self) -> f64 {
        let area = self.base * self.height;
        return area;
    }
}

//定义圆形计算函数：
impl Area for Circle {
    fn calculate(&self) -> f64 {
        let area = self.radius * self.radius * 3.14;
        return area;
    }
}

//定义三角形计算函数：
impl Area for Triangle {
    fn calculate(&self) -> f64 {
        let area = self.base * self.height;
        return area;
    }
}

//定义打印结果函数：
fn print_res<T: Area> (res: T) {
    println!("Area is {}.", res.calculate());
}



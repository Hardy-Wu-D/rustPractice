fn main() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width *  self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width >= other.width && self.height >= other.height
        }

        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }

        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }

    let rect1 = Rectangle { width: 10, height: 20 };
    let rect2 = Rectangle { width: 10, height: 10 };
    let rect3 = Rectangle { width: 20, height: 10 };
    let rect4 = Rectangle::square(20);

    println!("rec1 is：{:?}，area is {}, is square: {}", rect1, rect1.area(), rect1.is_square());
    println!("rec1 is：{:?}，area is {}, is square: {}", rect2, rect2.area(), rect2.is_square());
    println!("rec1 is：{:?}，area is {}, is square: {}", rect3, rect3.area(), rect3.is_square());
    println!("rect1 is：{:?}，area is {}, is square: {}", rect4, rect4.area(), rect4.is_square());
    println!("rect1 can hold rect2：{}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3：{}", rect1.can_hold(&rect3));
}

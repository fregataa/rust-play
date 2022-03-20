#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn new(width: u32, height: u32) -> Rect {
        return Rect {
            width,
            height,
        }
    }

    fn get_area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn main(){
    let rect = Rect::new(10, 4);

    println!("The area is {}", rect.get_area());
    // println!("{:?}", rect)
}

// fn calc_area(rect: Rect) -> u32 {
//     return rect.width * rect.height;
// }

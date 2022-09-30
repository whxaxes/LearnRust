#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    description: String,
}

fn build_user() -> User {
    User {
        name: String::from("axes"),
        age: 10,
        description: String::from("cool guy"),
    }
}

struct Retangle {
    width: u32,
    height: u32,
}

impl Retangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Retangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn new(width: u32, height: u32) -> Retangle {
        Retangle {
            width: width,
            height: height,
        }
    }
}

fn main() {
    let user = build_user();
    println!("{:#?}, {}: {}", user, user.name, user.description);

    let rt = Retangle {
        width: 100,
        height: 30,
    };

    println!("{}", rt.area());

    let rt2 = Retangle::new(10, 20);
    println!("{}", rt2.area());

    println!("{}", rt.can_hold(&rt2));
}



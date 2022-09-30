use rand::Rng;

#[derive(Debug)]
enum MyEnum {
    A,
    B(String),
    C { x: u8, y: u8 },
    D,
}

impl MyEnum {
    fn test_fn(&self) -> u8 {
        match &self {
            &Self::A => 1,
            &Self::B(_str) => 2,
            _ => 0,
        }
    }
}

fn main() {
    let a = MyEnum::A;
    println!("{:?}, {}", a, a.test_fn());

    let b = MyEnum::B(String::from("hello"));
    println!("{:?}", b);

    let c = MyEnum::C{ x: 1, y: 1 };
    println!("{:?}", c);

    println!("{}", test_enum(MyEnum::A));
    println!("{}", test_enum(MyEnum::B(String::from("hello"))));
    println!("{}", test_enum(MyEnum::C{ x: 1, y: 1 }));
    println!("{}", test_enum(MyEnum::D));

    println!("{}", test_enum_2(MyEnum::A));
    println!("{}", test_enum_2(MyEnum::B(String::from("hello"))));
    println!("{}", test_enum_2(MyEnum::C{ x: 1, y: 1 }));


    let arr = [ MyEnum::A, MyEnum::B(String::from("hello")), MyEnum::D ];
    let result = may_be_null(&arr[rand::thread_rng().gen_range(0..arr.len())]);
    match result {
        Some(n) => {
            println!("result is {}", n);
        }
        None => {
            println!("result is None");
        }
    }
}

fn test_enum_2(b: MyEnum) -> String {
    if let MyEnum::A = b {
        String::from("world")
    } else if let MyEnum::B(str) = b {
        str
    } else {
        String::from("any")
    }
}

fn test_enum(b: MyEnum) -> String {
    match b {
        MyEnum::A => String::from("world"),
        MyEnum::B(str) => str,
        MyEnum::C { x, y } => (x + y).to_string(),
        _ => String::from("any"),
    }
}

fn may_be_null(b: &MyEnum) -> Option<u32> {
    match b {
        MyEnum::A => Some(1),
        MyEnum::B(_str) => Some(2),
        _ => None,
    }
}
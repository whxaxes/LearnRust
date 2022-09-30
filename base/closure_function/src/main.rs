use std::error::Error;

fn main() {
    let test = |x| x;
    println!("{}", test(1));

    let a = String::from("1");
    let test2 = move |x: u32| -> Result<u32, Box<dyn Error>> {
        let s: u32 = a.parse()?;
        Ok(s + x)
    };
    println!("{}", test2(3).unwrap());

    let b = get_closure();
    println!("{}", b(1));

    let a2 = String::from("1");
    let test3 = move |x: u32| -> u32 {
        let r: u32 = a2.parse().unwrap();
        x + r
    };
    println!("{}", test3(3));
    test_closure();
}

fn get_closure() -> impl Fn(u32) -> u32 {
    let a = 123123;
    move |x: u32| -> u32 {
        a + x
    }
}

fn test_closure() {
    let mut a2 = 1;
    let test3 = move |x: u32| -> u32 {
        a2 + x
    };
    a2 = 3;
    println!("a2 = {}", a2);
    println!("a2 + x = {}", test3(1));
}

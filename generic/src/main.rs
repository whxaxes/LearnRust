use std::fmt::Debug;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn sqrt(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Man {
    name: String,
}

trait CanSay {
    fn say_hi(&self) -> ();

    fn say_yo(&self) {
        println!("yo!");
    }
}

trait CanWalk {
    fn walk(&self) {
        println!("walk walk walk");
    }
}

impl CanSay for Man {
    fn say_hi(&self) -> () {
        println!("hi, {}", self.name);
    }
}

impl CanWalk for Man {}

fn say_hi(m: &impl CanSay) {
    m.say_hi();
}

fn say_yo<T: CanSay>(m: &T) {
    m.say_yo();
}

fn say_yo_and_walk(m: &(impl CanSay + CanWalk)) {
    m.say_yo();
    m.walk();
}

fn say_hi_and_walk<T: CanSay + CanWalk>(m: &T) {
    m.say_hi();
    m.walk();
}

fn say_hi_and_walk_2<T>(m: &T) 
    where T: CanSay + CanWalk
{
    m.say_hi();
    m.walk();
}

fn should_return_walk() -> impl CanWalk + Debug {
    Man { name: String::from("111") }
}

fn should_return_say_walk() -> impl CanWalk + CanSay + Debug {
    Man { name: String::from("111") }
}

fn main() {
    let m = Man { name: String::from("axes") };
    m.say_hi();
    m.say_yo();

    say_hi(&m);
    say_yo(&m);
    say_hi_and_walk(&m);
    say_yo_and_walk(&m);
    say_hi_and_walk_2(&m);

    let m1 = should_return_walk();
    let m2 = should_return_say_walk();
    println!("{:?}, {:?}", m1, m2);

    let p = Point { x: 5, y: 10 };
    let r = p.x();
    println!("p.x = {}", r);

    let p2: Point<f32> = Point { x: 1.0, y: 2.0 };
    let r2 = p2.sqrt();
    println!("sqrt = {}", r2);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

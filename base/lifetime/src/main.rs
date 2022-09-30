struct Interface<'b, 'a> {
    manager: &'b mut Manager<'a>
}

impl<'b, 'a: 'b> Interface<'b, 'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'a mut self) -> Interface<'b, 'a> 
    where 'a: 'b {
        Interface {
            manager: &mut self.manager
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager {
            text: "hello"
        }
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了
    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}

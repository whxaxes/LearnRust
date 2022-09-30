mod mod_fn;
mod other;
mod common;
use other::mod_fn_2;

mod test_mod {
	pub fn test() -> u8 {
        1
    }
}

pub fn mylib3() -> u8 {
	let i = test_mod::test();
    i
}

pub fn mylib() -> u8 {
    let i = mod_fn::test();
    i
}

pub fn mylib2() -> u8 {
    let i = other::mod_fn_2::test();
    let i2 = mod_fn_2::test();
    let i3 = other::test();
    i + i2 + i3
}

#[cfg(test)]
mod tests {
    use crate::{mylib, mylib2, mylib3};

    #[test]
    fn it_works() {
        assert_eq!(mylib(), 1);
    }

    #[test]
    fn it_works2() {
        assert_eq!(mylib2(), 7);
    }

    #[test]
    fn it_works_test() {
        assert_eq!(mylib3(), 1);
    }
}

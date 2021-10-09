use crate::common::util;

pub fn test() -> u8 {
  util::test() + crate::test_mod::test()
}

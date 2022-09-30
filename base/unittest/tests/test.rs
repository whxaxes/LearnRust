use unittest;

mod util;

#[test]
fn it_works_in_single() {
  assert_eq!(unittest::mylib(), util::get_one())
}

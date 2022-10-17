use std::env;

pub struct TestUtil {}

impl TestUtil {
  pub fn wrap_with_label<F>(label: &str, f: F)
    where F: FnOnce()
  {
    let args: Vec<String> = env::args().collect();
    let mut grep: Option<&String> = None;
    'a: for (index, arg) in args.iter().enumerate() {
      if arg == "--grep" {
        grep = args.get(index + 1);
        break 'a;
      }
    }

    // 筛选一下
    let mut should_run = true;
    if let Some(g) = grep {
      should_run = label.contains(g);
    }

    if should_run {
      println!("\n========= {} start =========", label);
      f();
      println!("========= {} end ===========\n", label);
    }
  }
}

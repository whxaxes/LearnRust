use std::fs;
use std::error::Error;
use std::io::ErrorKind;

pub struct Config {
  pub query: String,
  pub filename: String,
}

pub struct SearchResult<'a> {
  pub line: usize,
  pub content: &'a str,
  pub underline: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      Err("not engough arguments")
    } else {
      Ok(Config {
        query: args[1].clone(),
        filename: args[2].clone(),
      })
    }
  }
}

pub fn search<'a>(query: &String, contents: &'a String) -> Vec<SearchResult<'a>> {
  let lower_query = query.to_lowercase();
  let mut list = Vec::new();
  for (i, str) in contents.lines().enumerate() {
    let lower_str = str.to_lowercase();
    let matches: Vec<(usize, &str)> = lower_str.match_indices(&lower_query).collect();
    if matches.len() > 0 {
      list.push(SearchResult {
        line: i + 1,
        content: str,
        underline: create_underline(&matches)
      });
    }
  }
  list
}

// 根据 match 的结果，返回 underline 内容
pub fn create_underline(matches: &Vec<(usize, &str)>) -> String {
  let mut underline = String::new();
  let mut index = 0;
  for (match_index, match_str) in matches {
    let space_len = match_index - index;
    underline.push_str(&format!("{}{}", " ".repeat(space_len), "^".repeat(match_str.len())));
    index = *match_index + match_str.len();
  }
  underline
}

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
  let contents = fs::read_to_string(&config.filename)
    .or_else(|err| {
      if err.kind() == ErrorKind::NotFound {
        Err(format!("failed to read file: {}", config.filename))
      } else {
        Err(format!("{:?}", err))
      }
    })?;

  let result = search(&config.query, &contents);

  // 格式化输出结果
  let mut str_result = String::new();
  str_result.push_str(&format!("\n  Search {} in {}\n", config.query, config.filename));
  for r in result {
    let prefix = format!("{}{}", " ".repeat(6 - r.line.to_string().len()), r.line);
    str_result.push_str(&format!("\n  {}. |   {}\n", prefix, r.content));
    str_result.push_str(&format!("  {}  |   {}", " ".repeat(prefix.len()), r.underline));
  }
  str_result.push_str("\n");

  Ok(str_result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn config_should_works() {
    let args = [ String::from("a") ];
    let conf = Config::new(&args);
    match conf {
        Err(e) => {
          assert_eq!(e, "not engough arguments");
        },
        _ => panic!("should throw error")
    }
  }

  #[test]
  fn should_search_without_error() {
    let mut query = String::from("duct");
    let contents = "
Rust:
safe, fast, productive.
Pick three.
      ".to_string();

    let result = search(&query, &contents);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].line, 3);
    assert_eq!(result[0].content, "safe, fast, productive.");
    assert_eq!(result[0].underline, "               ^^^^");

    query.clear();
    query.push_str("u");
    let result2 = search(&query, &contents);
    assert_eq!(result2.len(), 2);
    assert_eq!(result2[0].line, 2);
    assert_eq!(result2[0].content, "Rust:");
    assert_eq!(result2[1].line, 3);
    assert_eq!(result2[1].content, "safe, fast, productive.");
  }
}
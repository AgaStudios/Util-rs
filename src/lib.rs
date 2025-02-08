mod _tokenize;
pub mod list;

pub use _tokenize::*;
pub use list::*;
pub fn is_valid_char(valid_chars: &str, eval_char: char) -> bool {
  for c in valid_chars.chars() {
    if c == eval_char {
      return true;
    }
  }
  false
}
pub fn get_content(text: &str, start: Position, end: Position) -> Option<String> {
  if start >= end {
    return None;
  }
  let mut string = String::new();
  let lines = text.lines();
  for (line_number, line) in lines.enumerate() {
    if start.line < line_number {
      continue;
    }
    if end.line > line_number {
      break;
    }
    for (char_number, char) in line.char_indices() {
      if start.column < char_number {
        continue;
      }
      if end.line == line_number && end.column > char_number {
        break;
      }
      string.push(char);
    }
  }
  Some(string)
}

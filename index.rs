
fn main() {
}

pub fn select(mut token: &str) -> &str {
  match token {
    "box1" => token = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏",
    _      => token = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏",
  }

  return token;
}

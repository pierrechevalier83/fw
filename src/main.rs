fn main() {
    let fw = |c: char| if c.is_alphabetic() {
        std::char::from_u32(c as u32 + 65248)
    } else {
         match c { ' ' => Some('　'), _ => Some(c), }
    };
	let pretty_print = |mut s: String| println!("{}", s.drain(..).filter_map(&fw).collect::<String>());
    if !std::env::args().skip(1).map(&pretty_print).collect::<Vec<_>>().is_empty() { return }
    let flatten_line = |line: std::io::Result<_>| line.unwrap_or("".to_string());
    use std::io::BufRead;
    let stdin = std::io::stdin();
    stdin.lock().lines().map(&flatten_line).map(&pretty_print).collect::<Vec<_>>();
}

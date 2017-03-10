fn fw(s: &mut String) -> String {
    s.drain(..)
        .filter_map(|c| if c.is_alphabetic() {
                        std::char::from_u32(c as u32 + 65248)
                    } else {
                        match c {
                            ' ' => Some('　'),
                            _ => Some(c),
                        }
                    })
        .collect()
}

fn main() {
    let pretty_print = |mut s| {
        print!("{}", fw(&mut s));
    };
    let args = std::env::args().skip(1).map(pretty_print).collect::<Vec<_>>();
    if args.is_empty() {
        use std::io::BufRead;
        let stdin = std::io::stdin();
        stdin.lock()
            .lines()
            .map(|line| line.unwrap_or("".to_string()) + "\n")
            .map(pretty_print)
            .collect::<Vec<_>>();
    }
}

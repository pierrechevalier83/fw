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
    let args = std::env::args()
        .skip(1)
        .map(|mut s| {
                 print!("{}", fw(&mut s));
             })
        .collect::<Vec<_>>();
    if args.is_empty() {
        let mut input = String::new();
        loop {
            if let Ok(_) = std::io::stdin().read_line(&mut input) {
                print!("{}", fw(&mut input));
            }
        }
    }
}

fn create_scream(max_len: usize) -> String {
    let scream_len = fastrand::usize(..=max_len);
    let excl_len = max_len - scream_len;

    format!("{}{}", "A".repeat(scream_len), "!".repeat(excl_len))
}

fn main() {
    let mut numargs = std::env::args().filter_map(|arg| arg.parse::<usize>().ok());
    if let Some(chars) = numargs.next() {
        println!("{}", create_scream(chars));
    } else {
        println!("Not screaming today.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_scream() {
        let len = 10;
        let scream = create_scream(len);

        assert_eq!(len, scream.len())
    }
}

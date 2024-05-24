use std::io::{stdin, stdout, Write};

pub fn clean_cli_args() -> Vec<String> {
    return clean_args(std::env::args().into_iter().skip(1));
}

fn clean_args<I>(args: I) -> Vec<String> where I: Iterator<Item = String> {
    return args.flat_map(|x| {
        if x.starts_with("--") || !x.starts_with("-") {
            vec![x]
        }
        else {
            x.chars().into_iter().skip(1).map(|x| format!("-{}", x)).collect()
        }
    }).collect();
}
pub fn get_cli_input(msg: &str) -> String {
    print!("{}", msg);
    let _ = stdout().flush();
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Could not read user input");
    println!();
    return input.trim().into();
}

#[cfg(test)]
mod tests {
    #![allow(warnings)]
    use super::*;

    #[test]
    fn clean_cli_args() {
        assert_eq!(
            clean_args(["-abc".into(), "--def".into(), "ghi".into()].into_iter()), 
            vec!["-a", "-b", "-c", "--def", "ghi"]
        );
    }
}

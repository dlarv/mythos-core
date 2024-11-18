use mythos_core::cli::clean_cli_args;

fn main() {
    println!("{args}", args=clean_cli_args().collect::<Vec<String>>().join(" "));
}

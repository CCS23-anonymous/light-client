mod helper;

fn main() {
    let log_n = helper::parse_args_or(8, "basic");
    println!("Running test for the 'basic' scheme for N = 2^{}", log_n);
    light_client::test_helpers::test_simple_scheme(log_n);
}
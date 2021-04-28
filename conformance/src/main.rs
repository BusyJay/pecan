use conformance::do_test_io;
use std::io;

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut test_count = 0;
    while do_test_io(&mut stdin, &mut stdout).unwrap() {
        test_count += 1;
    }
    eprintln!(
        "conformance-rust: received EOF from test runner after {} tests, existing",
        test_count
    );
}

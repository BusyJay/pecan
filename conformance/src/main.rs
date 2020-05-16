use conformance::*;

fn main() {
    let mut test_count = 0;
    loop {
        match do_test_io() {
            Ok(false) => {
                test_count += 1;
            },
            Ok(true) => {
                eprintln!("conformance_rust: {} tests complete, exit", test_count);
                break;
            }
            Err(e) => {
                panic!("conformance_rust: received {:?} from test runner after {} tests, exiting", e, test_count);
            }
        }
    }
}

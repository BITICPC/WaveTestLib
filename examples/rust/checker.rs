extern crate wave_test_lib;

fn main() {
    let checker = wave_test_lib::Checker::new();
    let std_ans = checker.std_answer();
    let usr_ans = checker.user_answer();
    
    while let Some(expected) = std_ans.read_token_as::<f64>() {
        usr_ans.expect_float_eq(expected, 1e-8);
    }

    usr_ans.expect_eof();
}

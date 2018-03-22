extern crate copy_from_str;
#[macro_use]
extern crate quickcheck;

use copy_from_str::CopyFromStrExt;
use quickcheck::TestResult;
use std::str;

quickcheck! {
    fn prop(out: String, src: String) -> TestResult {
        if out.len() != src.len() {
            return TestResult::discard();
        }
        let mut out = out;
        out.copy_from_str(&src);
        println!("{:?} {:?}", out, src);
        TestResult::from_bool(out == src && str::from_utf8(out.as_bytes()).is_ok())
    }
}

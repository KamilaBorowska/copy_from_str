extern crate copy_from_str;
#[macro_use]
extern crate quickcheck;

use copy_from_str::CopyFromStrExt;
use quickcheck::TestResult;

quickcheck! {
    fn prop(out: String, src: String) -> TestResult {
        if out.len() != src.len() {
            return TestResult::discard();
        }
        let mut out = out;
        out.copy_from_str(&src);
        TestResult::from_bool(out == src)
    }
}

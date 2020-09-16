extern crate copy_from_str;
#[macro_use]
extern crate quickcheck;

use copy_from_str::CopyFromStrExt;
use quickcheck::TestResult;

quickcheck! {
    fn prop(out: String, src: String) -> TestResult {
        let range = ..out.len().min(src.len());
        let mut out = out;
        if let (Some(out), Some(src)) = (out.get_mut(range), src.get(range)) {
            out.copy_from_str(&src);
            TestResult::from_bool(out == src)
        } else {
            TestResult::discard()
        }
    }
}

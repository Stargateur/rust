// compile-flags: -Zunleash-the-miri-inside-of-you
#![feature(box_syntax)]
#![allow(const_err)]

use std::mem::ManuallyDrop;

fn main() {}

static TEST_BAD: &mut i32 = {
    &mut *(box 0)
    //~^ WARN skipping const check
    //~| ERROR could not evaluate static initializer
    //~| NOTE heap allocations
    //~| WARN skipping const checks
    //~| WARN skipping const checks
    //~| WARN skipping const checks
};

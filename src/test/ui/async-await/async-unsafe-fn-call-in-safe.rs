// edition:2018

#![feature(async_await)]

struct S;

impl S {
    async unsafe fn f() {}
}

async unsafe fn f() {}

async fn g() {
    S::f(); //~ ERROR call to unsafe function is unsafe
    f(); //~ ERROR call to unsafe function is unsafe
}

fn main() {
    S::f(); //~ ERROR call to unsafe function is unsafe
    f(); //~ ERROR call to unsafe function is unsafe
}

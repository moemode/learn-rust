static mut STASH: &i32 = &128;
static MY_INTEGER: i32 = 42;
fn main() {
    f(&MY_INTEGER);
}

/**
 * Conversely, if we do see a function with a signature 
 * like g(p: &i32) (or with the lifetimes written out, g<'a>(p: &'a i32)), we can tell that it does not stash its argument p anywhere that will outlive the call.
 */
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

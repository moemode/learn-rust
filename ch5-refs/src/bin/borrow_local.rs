fn main() {
    let r;
    {
        let x = 1;
        r = &x;
        // the reference’s lifetime must be contained by x’s, but fully enclose r’s
    }
    assert_eq!(*r, 1);
}

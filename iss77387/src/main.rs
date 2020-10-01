fn main() {
    let s = 0 as *const usize;
    let f = unsafe {
        {
            *s
        }
    };
    eprintln!("{:?}", f(s)); // DEBUG
}

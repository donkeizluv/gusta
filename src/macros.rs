#[macro_export]
macro_rules! cls {
    () => {
        print!("\x1B[2J\x1B[1;1H");
    };
}

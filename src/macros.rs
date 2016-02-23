
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_unreachable {
    () => {
        unreachable!()
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_unreachable {
    () => {
        ()
    };
}


#[test]
#[should_panic]
fn panics() {
    debug_unreachable!();
}

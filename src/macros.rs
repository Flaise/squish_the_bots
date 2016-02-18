
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_unreachable {
    () => {
        unreachable!()
    };
    (return) => {
        unreachable!()
    };
    (break) => {
        unreachable!()
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_unreachable {
    () => {
        ()
    };
    (return) => {
        return
    };
    (break) => {
        break
    };
}


#[test]
#[should_panic]
fn panics() {
    debug_unreachable!();
}

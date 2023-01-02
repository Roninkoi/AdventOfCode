#[cfg(debug_assertions)]
macro_rules! dprintln {
    ($($arg:tt)*) => {{
        println!($($arg)*);
    }};
}
#[cfg(not(debug_assertions))]
macro_rules! dprintln {
    ($($arg:tt)*) => {{}};
}
#[cfg(debug_assertions)]
macro_rules! dprint {
    ($($arg:tt)*) => {{
        print!($($arg)*);
    }};
}
#[cfg(not(debug_assertions))]
macro_rules! dprint {
    ($($arg:tt)*) => {{}};
}

pub(crate) use dprint;
pub(crate) use dprintln;

#[macro_export]
macro_rules! color {
    ($c:expr, $str:expr) => {
        concat!("\x1b[", $c, "m", $str, "\x1b[0m")
    };
}

#[macro_export]
macro_rules! c {
    (black, $str:expr) => {
        crate::color!("30", $str)
    };
    (red, $str:expr) => {
        crate::color!("31", $str)
    };
    (green, $str:expr) => {
        crate::color!("32", $str)
    };
    (yellow, $str:expr) => {
        crate::color!("33", $str)
    };
    (blue, $str:expr) => {
        crate::color!("34", $str)
    };
    (magenta, $str:expr) => {
        crate::color!("35", $str)
    };
    (cyan, $str:expr) => {
        crate::color!("36", $str)
    };
    (white, $str:expr) => {
        crate::color!("37", $str)
    };
}

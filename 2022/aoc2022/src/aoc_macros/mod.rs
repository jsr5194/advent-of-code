macro_rules! aoc_println {
    // macth like arm for macro
    ($msg:expr) => {
        // macro expand to this code
        if DEBUG {
            println!("{:?}", $msg);
        }
    };
}

pub(crate) use aoc_println;

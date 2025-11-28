#[macro_export]
macro_rules! verbose {
    ($opts:expr, $($arg:tt)*) => {
        if $opts.verbose {
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! separator {
    ($item:expr, $collection:expr) => {
        use colored::Colorize;
        if $collection.len() > 1 {
            println!("\n{}\n", $item.as_str().bold());
        }
    };
}

#[macro_export]
macro_rules! err_if_empty {
    ($flist:expr) => {
        anyhow::ensure!($flist.len() > 0, "nothing on which to operate")
    };
}

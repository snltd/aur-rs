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

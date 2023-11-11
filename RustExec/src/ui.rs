macro_rules! warn {
    ($fmt: literal, $ex: expr) => {
        use console::{style, Emoji};
        use std::env;
        let formatstr = format!($fmt, $ex);
        if env::var("NO_EMOJI").is_ok() {
            println!("{} {}", style("").red(), style(formatstr).red())
        } else {
            println!("{} {}", style("⚠️", "!").red(), style(formatstr).red());
        }
    };
}

macro_rules! success {
    ($fmt: literal, $ex: expr) => {
        use console::{style, Emoji};
        use std::env;
        let formatstr = format!($fmt, $ex);
        if env::var("NO_EMOJI").is_ok() {
            println!("{} {}", style("").green(), style(formatstr).green())
        } else {
            println!("{} {}", style("⚠️", "!").green(), style(formatstr).green());
        }
    };
}

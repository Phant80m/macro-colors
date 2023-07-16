pub trait Colorize {
    // Regular colors
    fn black(&self) -> String;
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
    fn magenta(&self) -> String;
    fn cyan(&self) -> String;
    fn white(&self) -> String;

    // Bright colors
    fn bright_black(&self) -> String;
    fn bright_red(&self) -> String;
    fn bright_green(&self) -> String;
    fn bright_yellow(&self) -> String;
    fn bright_blue(&self) -> String;
    fn bright_magenta(&self) -> String;
    fn bright_cyan(&self) -> String;
    fn bright_white(&self) -> String;

    // Background colors
    fn on_black(&self) -> String;
    fn on_red(&self) -> String;
    fn on_green(&self) -> String;
    fn on_yellow(&self) -> String;
    fn on_blue(&self) -> String;
    fn on_magenta(&self) -> String;
    fn on_cyan(&self) -> String;
    fn on_white(&self) -> String;

    // Text styles
    fn bold(&self) -> String;
    fn italic(&self) -> String;
}

impl Colorize for String {
    fn black(&self) -> String {
        format!("\x1B[30m{}\x1B[0m", self)
    }

    fn red(&self) -> String {
        format!("\x1B[31m{}\x1B[0m", self)
    }

    fn green(&self) -> String {
        format!("\x1B[32m{}\x1B[0m", self)
    }

    fn yellow(&self) -> String {
        format!("\x1B[33m{}\x1B[0m", self)
    }

    fn blue(&self) -> String {
        format!("\x1B[34m{}\x1B[0m", self)
    }

    fn magenta(&self) -> String {
        format!("\x1B[35m{}\x1B[0m", self)
    }

    fn cyan(&self) -> String {
        format!("\x1B[36m{}\x1B[0m", self)
    }

    fn white(&self) -> String {
        format!("\x1B[37m{}\x1B[0m", self)
    }

    fn bright_black(&self) -> String {
        format!("\x1B[30;1m{}\x1B[0m", self)
    }

    fn bright_red(&self) -> String {
        format!("\x1B[31;1m{}\x1B[0m", self)
    }

    fn bright_green(&self) -> String {
        format!("\x1B[32;1m{}\x1B[0m", self)
    }

    fn bright_yellow(&self) -> String {
        format!("\x1B[33;1m{}\x1B[0m", self)
    }

    fn bright_blue(&self) -> String {
        format!("\x1B[34;1m{}\x1B[0m", self)
    }

    fn bright_magenta(&self) -> String {
        format!("\x1B[35;1m{}\x1B[0m", self)
    }

    fn bright_cyan(&self) -> String {
        format!("\x1B[36;1m{}\x1B[0m", self)
    }

    fn bright_white(&self) -> String {
        format!("\x1B[37;1m{}\x1B[0m", self)
    }

    fn on_black(&self) -> String {
        format!("\x1B[40m{}\x1B[0m", self)
    }

    fn on_red(&self) -> String {
        format!("\x1B[41m{}\x1B[0m", self)
    }

    fn on_green(&self) -> String {
        format!("\x1B[42m{}\x1B[0m", self)
    }

    fn on_yellow(&self) -> String {
        format!("\x1B[43m{}\x1B[0m", self)
    }

    fn on_blue(&self) -> String {
        format!("\x1B[44m{}\x1B[0m", self)
    }

    fn on_magenta(&self) -> String {
        format!("\x1B[45m{}\x1B[0m", self)
    }

    fn on_cyan(&self) -> String {
        format!("\x1B[46m{}\x1B[0m", self)
    }

    fn on_white(&self) -> String {
        format!("\x1B[47m{}\x1B[0m", self)
    }

    fn bold(&self) -> String {
        format!("\x1B[1m{}\x1B[0m", self)
    }

    fn italic(&self) -> String {
        format!("\x1B[3m{}\x1B[0m", self)
    }
}

impl<'a> Colorize for &'a str {
    fn black(&self) -> String {
        format!("\x1B[30m{}\x1B[0m", self)
    }

    fn red(&self) -> String {
        format!("\x1B[31m{}\x1B[0m", self)
    }

    fn green(&self) -> String {
        format!("\x1B[32m{}\x1B[0m", self)
    }

    fn yellow(&self) -> String {
        format!("\x1B[33m{}\x1B[0m", self)
    }

    fn blue(&self) -> String {
        format!("\x1B[34m{}\x1B[0m", self)
    }

    fn magenta(&self) -> String {
        format!("\x1B[35m{}\x1B[0m", self)
    }

    fn cyan(&self) -> String {
        format!("\x1B[36m{}\x1B[0m", self)
    }

    fn white(&self) -> String {
        format!("\x1B[37m{}\x1B[0m", self)
    }

    fn bright_black(&self) -> String {
        format!("\x1B[30;1m{}\x1B[0m", self)
    }

    fn bright_red(&self) -> String {
        format!("\x1B[31;1m{}\x1B[0m", self)
    }

    fn bright_green(&self) -> String {
        format!("\x1B[32;1m{}\x1B[0m", self)
    }

    fn bright_yellow(&self) -> String {
        format!("\x1B[33;1m{}\x1B[0m", self)
    }

    fn bright_blue(&self) -> String {
        format!("\x1B[34;1m{}\x1B[0m", self)
    }

    fn bright_magenta(&self) -> String {
        format!("\x1B[35;1m{}\x1B[0m", self)
    }

    fn bright_cyan(&self) -> String {
        format!("\x1B[36;1m{}\x1B[0m", self)
    }

    fn bright_white(&self) -> String {
        format!("\x1B[37;1m{}\x1B[0m", self)
    }

    fn on_black(&self) -> String {
        format!("\x1B[40m{}\x1B[0m", self)
    }

    fn on_red(&self) -> String {
        format!("\x1B[41m{}\x1B[0m", self)
    }

    fn on_green(&self) -> String {
        format!("\x1B[42m{}\x1B[0m", self)
    }

    fn on_yellow(&self) -> String {
        format!("\x1B[43m{}\x1B[0m", self)
    }

    fn on_blue(&self) -> String {
        format!("\x1B[44m{}\x1B[0m", self)
    }

    fn on_magenta(&self) -> String {
        format!("\x1B[45m{}\x1B[0m", self)
    }

    fn on_cyan(&self) -> String {
        format!("\x1B[46m{}\x1B[0m", self)
    }

    fn on_white(&self) -> String {
        format!("\x1B[47m{}\x1B[0m", self)
    }

    fn bold(&self) -> String {
        format!("\x1B[1m{}\x1B[0m", self)
    }

    fn italic(&self) -> String {
        format!("\x1B[3m{}\x1B[0m", self)
    }
}

impl<'a> Colorize for &'a mut str {
    fn black(&self) -> String {
        format!("\x1B[30m{}\x1B[0m", self)
    }

    fn red(&self) -> String {
        format!("\x1B[31m{}\x1B[0m", self)
    }

    fn green(&self) -> String {
        format!("\x1B[32m{}\x1B[0m", self)
    }

    fn yellow(&self) -> String {
        format!("\x1B[33m{}\x1B[0m", self)
    }

    fn blue(&self) -> String {
        format!("\x1B[34m{}\x1B[0m", self)
    }

    fn magenta(&self) -> String {
        format!("\x1B[35m{}\x1B[0m", self)
    }

    fn cyan(&self) -> String {
        format!("\x1B[36m{}\x1B[0m", self)
    }

    fn white(&self) -> String {
        format!("\x1B[37m{}\x1B[0m", self)
    }

    fn bright_black(&self) -> String {
        format!("\x1B[30;1m{}\x1B[0m", self)
    }

    fn bright_red(&self) -> String {
        format!("\x1B[31;1m{}\x1B[0m", self)
    }

    fn bright_green(&self) -> String {
        format!("\x1B[32;1m{}\x1B[0m", self)
    }

    fn bright_yellow(&self) -> String {
        format!("\x1B[33;1m{}\x1B[0m", self)
    }

    fn bright_blue(&self) -> String {
        format!("\x1B[34;1m{}\x1B[0m", self)
    }

    fn bright_magenta(&self) -> String {
        format!("\x1B[35;1m{}\x1B[0m", self)
    }

    fn bright_cyan(&self) -> String {
        format!("\x1B[36;1m{}\x1B[0m", self)
    }

    fn bright_white(&self) -> String {
        format!("\x1B[37;1m{}\x1B[0m", self)
    }

    fn on_black(&self) -> String {
        format!("\x1B[40m{}\x1B[0m", self)
    }

    fn on_red(&self) -> String {
        format!("\x1B[41m{}\x1B[0m", self)
    }

    fn on_green(&self) -> String {
        format!("\x1B[42m{}\x1B[0m", self)
    }

    fn on_yellow(&self) -> String {
        format!("\x1B[43m{}\x1B[0m", self)
    }

    fn on_blue(&self) -> String {
        format!("\x1B[44m{}\x1B[0m", self)
    }

    fn on_magenta(&self) -> String {
        format!("\x1B[45m{}\x1B[0m", self)
    }

    fn on_cyan(&self) -> String {
        format!("\x1B[46m{}\x1B[0m", self)
    }

    fn on_white(&self) -> String {
        format!("\x1B[47m{}\x1B[0m", self)
    }

    fn bold(&self) -> String {
        format!("\x1B[1m{}\x1B[0m", self)
    }

    fn italic(&self) -> String {
        format!("\x1B[3m{}\x1B[0m", self)
    }
}

#[macro_export]
macro_rules! red_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).red()));
    }}
}

#[macro_export]
macro_rules! bold_red_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).red().bold()));
    }}
}

#[macro_export]
macro_rules! green_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).green()));
    }}
}

#[macro_export]
macro_rules! bold_green_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).green().bold()));
    }}
}

#[macro_export]
macro_rules! yellow_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).yellow()));
    }}
}

#[macro_export]
macro_rules! bold_yellow_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).yellow().bold()));
    }}
}

#[macro_export]
macro_rules! blue_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).blue()));
    }}
}

#[macro_export]
macro_rules! bold_blue_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).blue().bold()));
    }}
}

#[macro_export]
macro_rules! purple_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).purple()));
    }}
}

#[macro_export]
macro_rules! bold_purple_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).purple().bold()));
    }}
}

#[macro_export]
macro_rules! cyan_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).cyan()));
    }}
}

#[macro_export]
macro_rules! bold_cyan_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).cyan().bold()));
    }}
}

#[macro_export]
macro_rules! white_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).white()));
    }}
}

#[macro_export]
macro_rules! bold_white_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).white().bold()));
    }}
}
#[macro_export]
macro_rules! italic_red_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).red().italic()));
    }}
}

#[macro_export]
macro_rules! italic_green_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).green().italic()));
    }}
}

#[macro_export]
macro_rules! italic_yellow_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).yellow().italic()));
    }}
}

#[macro_export]
macro_rules! italic_blue_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).blue().italic()));
    }}
}

#[macro_export]
macro_rules! italic_purple_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).purple().italic()));
    }}
}

#[macro_export]
macro_rules! italic_cyan_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).cyan().italic()));
    }}
}
#[macro_export]
macro_rules! italic_white_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).white().italic()));
    }}
}
#[macro_export]
macro_rules! bold_italic_red_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).red().bold().italic()));
    }}
}

#[macro_export]
macro_rules! bold_italic_green_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).green().bold().italic()));
    }}
}

#[macro_export]
macro_rules! bold_italic_yellow_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).yellow().bold().italic()));
    }}
}

#[macro_export]
macro_rules! bold_italic_blue_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).blue().bold().italic()));
    }}
}

#[macro_export]
macro_rules! bold_italic_purple_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).purple().bold().italic()));
    }}
}

#[macro_export]
macro_rules! bold_italic_cyan_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).cyan().bold().italic()));
    }}
}

#[macro_export]
macro_rules! bold_italic_white_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).white().bold().italic()));
    }}
}
#[macro_export]
macro_rules! bg_red_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_red()));
    }}
}

#[macro_export]
macro_rules! bg_green_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_green()));
    }}
}

#[macro_export]
macro_rules! bg_yellow_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_yellow()));
    }}
}

#[macro_export]
macro_rules! bg_blue_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_blue()));
    }}
}

#[macro_export]
macro_rules! bg_purple_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_purple()));
    }}
}

#[macro_export]
macro_rules! bg_cyan_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_cyan()));
    }}
}

#[macro_export]
macro_rules! bg_white_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_white()));
    }}
}
#[macro_export]
macro_rules! bold_bg_red_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_red().bold()));
    }}
}

#[macro_export]
macro_rules! bold_bg_green_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_green().bold()));
    }}
}

#[macro_export]
macro_rules! bold_bg_yellow_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_yellow().bold()));
    }}
}

#[macro_export]
macro_rules! bold_bg_blue_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_blue().bold()));
    }}
}

#[macro_export]
macro_rules! bold_bg_purple_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_purple().bold()));
    }}
}

#[macro_export]
macro_rules! bold_bg_cyan_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_cyan().bold()));
    }}
}

#[macro_export]
macro_rules! bold_bg_white_println {
    ($($arg:tt)*) => {{
        println!("{}", format_args!("{}", format!($($arg)*).on_white().bold()));
    }}
}

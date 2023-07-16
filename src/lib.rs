pub use owo_colors::OwoColorize;

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

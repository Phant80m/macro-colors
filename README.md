# macro_colors
Super simple rust color macros!
Custom Macros for Colored Printing

These macros are custom printing macros designed to make terminal output colorful and stylized. They are helpful for adding colors, bold, and italic styles to printed text.

Traits Colorize:
- Works on `Strings`, `mut &str`, and `&str` 
```rs
.bold()
.italic()
.black()
.red()
.yellow()
.blue()
.magenta()
.cyan()
.white()
.bright_black()
.bright_red()
.bright_yellow()
.bright_blue()
.bright_magenta()
.bright_cyan()
.bright_white()
.on_black()
.on_red()
.on_yellow()
.on_blue()
.on_magenta()
.on_cyan()
.on_white()
```

Text color Macros:
```rs
 red_println!("Example text")
 green_println!("Example text")
 yellow_println!("Example text")
 blue_println!("Example text")
 purple_println!("Example text")
 cyan_println!("Example text")
 white_println!("Example text")
```
Bold Text Color Macros:

```rs
 bold_red_println!("Example text")
 bold_green_println!("Example text")
 bold_yellow_println!("Example text")
 bold_blue_println!("Example text")
 bold_purple_println!("Example text")
 bold_cyan_println!("Example text")
 bold_white_println!("Example text")
```
Italic Text Color Macros:
```rs
 italic_red_println!("Example text")
 italic_green_println!("Example text")
 italic_yellow_println!("Example text")
 italic_blue_println!("Example text")
 italic_purple_println!("Example text")
 italic_cyan_println!("Example text")
 italic_white_println!("Example text")
```
Bold and Italic Text Color Macros:
```rs
 bold_italic_red_println!("Example text")
 bold_italic_green_println!("Example text")
 bold_italic_yellow_println!("Example text")
 bold_italic_blue_println!("Example text")
 bold_italic_purple_println!("Example text")
 bold_italic_cyan_println!("Example text")
 bold_italic_white_println!("Example text")
```
Background Color Macros:
```rs
 bg_red_println!("Example text")
 bg_green_println!("Example text") 
 bg_yellow_println!("Example text")
 bg_blue_println!("Example text")
 bg_purple_println!("Example text")
 bg_cyan_println!("Example text") 
 bg_white_println!("Example text")
```
Bold Background Color Macros:
```rs
 bold_bg_red_println!("Example text")
 bold_bg_green_println!("Example text")
 bold_bg_yellow_println!("Example text")
 bold_bg_blue_println!("Example text")
 bold_bg_purple_println!("Example text")
 bold_bg_cyan_println!("Example text")
 bold_bg_white_println!("Example text")
```



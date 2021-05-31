extern crate win32console;
extern crate winrt_notification;

use std::{process::exit, time::Duration};

use winrt_notification::Toast;

const TM_BOLD: &'static str = "1";
const TM_NEGATIVE: &'static str = "7";
const TM_POSITIVE: &'static str = "27";
const TM_DISABLE_BOLD: &'static str = "22";
const TM_BLACK: &'static str = "30";
const TM_RED: &'static str = "31";
const _TM_BLUE: &'static str = "34";
const TM_DEFAULT_COLOR: &'static str = "39";

const TIME_STEP: f32 = 1. / 60.;

fn main() {
    let args:Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid number of arguments.");
        print_usage_and_quit();
    }

    let duration;
    let mut reminder_duration = None;

    if let Ok(parsed_duration) = args[1].parse::<f32>() {
        duration = parsed_duration;
    } else {
        let parts:Vec<&str> = args[1].split('+').collect();
        if parts.len() != 2 {
            eprintln!("Invalid duration/reminder argument.");
            print_usage_and_quit();
            return;
        }
        if let Ok(parsed_duration) = parts[0].parse::<f32>() {
            duration = parsed_duration;
            if let Ok(parsed_reminder_duration) = parts[1].parse::<f32>() {
                reminder_duration = Some(parsed_reminder_duration);
            } else {
                eprintln!("Failed to parse reminder duration.");
                print_usage_and_quit();
            }
        } else {
            eprintln!("Failed to parse duration.");
            print_usage_and_quit();
            return;
        }
    }
    
    let mut remaining_duration = duration;
    
    clear_console();
    hide_cursor();
    print_remaining(remaining_duration, duration);
    while remaining_duration >= TIME_STEP {
        std::thread::sleep(Duration::from_secs(1));
        remaining_duration -= TIME_STEP;
        print_remaining(remaining_duration, duration);
    }

    std::thread::sleep(Duration::from_secs_f32(remaining_duration));
    
    let alert_msg = format!("{} minutes over!", duration);
    let repeat_msg = reminder_duration.map(|rd| format!("Repeats in {} minutes.", rd));
    show_toast(&alert_msg, &repeat_msg, true);

    if let Some(rd) = reminder_duration {
        loop {
            std::thread::sleep(Duration::from_secs_f32(rd * 60.));
            show_toast(&alert_msg, &repeat_msg, false);
        }
    }
}

fn show_toast(title: &str, text: &Option<String>, print_title: bool) {
    if print_title {
        clear_console();
        text_mode(TM_BOLD);
        text_mode(TM_RED);
        println!("{}", title);
        text_mode(TM_DEFAULT_COLOR);
        text_mode(TM_DISABLE_BOLD);
    }

    let mut temp_toast = Toast::new(Toast::POWERSHELL_APP_ID).title(title);

    if let Some(content) = text {
        temp_toast = temp_toast.text1(&content);
        text_mode(TM_BOLD);
        text_mode(TM_BLACK);
        println!("{}", &content);
        text_mode(TM_DEFAULT_COLOR);
        text_mode(TM_DISABLE_BOLD);
    }

    temp_toast
        .duration(winrt_notification::Duration::Short)
        .sound(Some(winrt_notification::Sound::SMS))
        .show()
        .unwrap();
}

fn print_remaining(remaining_duration: f32, total_duration: f32) {
    set_cursor_position(1, 1);
    let console_width = win32console::console::WinConsole::output().get_screen_buffer_info().unwrap().screen_buffer_size.x as u32;
    text_mode(TM_BOLD);

    let fill_width =  (console_width as f32      * (remaining_duration as f32 / total_duration as f32)      ) as u32;
    let sub_width =  ((console_width as f32 * 8. * (remaining_duration as f32 / total_duration as f32)) % 8.) as u32;

    let text = if remaining_duration <= 1. {
        format!("{} seconds remaining", (remaining_duration * 60.) as u32)
    } else {
        format!("{} minutes remaining", (remaining_duration + 0.5) as u32)
    };

    let side_space_left = u32::max((console_width - text.len() as u32) / 2, 0);
    let side_space_right = if ((console_width as f32 - text.len() as f32) / 2.) % 1. >= 0.5 { side_space_left + 1 } else { side_space_left };
    // Top Row
    for _ in 0..console_width {
        print!("▁");
    }
    println!();

    // Content Row
    text_mode(TM_NEGATIVE);

    // Spaces left of text
    for i in 0..side_space_left {
        if i == fill_width {
            text_mode(TM_POSITIVE);
            print_sub_char(sub_width);
        } else {
            print!(" ");
        }
    }

    // Text
    let mut ci = side_space_left;
    for c in text.chars() {
        if ci == fill_width { text_mode(TM_POSITIVE); }
        print!("{}", c);
        ci += 1;
    }

    // Spaces right of text
    for i in 0..side_space_right-1 {
        if side_space_left + text.len() as u32 + i == fill_width {
            text_mode(TM_POSITIVE);
            print_sub_char(sub_width);
        } else {
            print!(" ");
        }
    }

    // End
    text_mode(TM_POSITIVE);
    if fill_width == console_width {
        print!("█");
    } else {
        print!("▕");
    }
    println!();

    // Bot Row
    for _ in 0..console_width {
        print!("▔");
    }
    println!();
    clear_line();

    text_mode(TM_DISABLE_BOLD);
}

fn print_sub_char(sub_width: u32) {
    let sub_char = match sub_width {
        1 => "▏",
        2 => "▎",
        3 => "▍",
        4 => "▌",
        5 => "▋",
        6 => "▊",
        7 => "▉",
        _ => " ",
    };
    print!("{}", &sub_char);
}

fn set_cursor_position(x: u32, y: u32) {
    print!("\x1B[{};{}H", x, y);
}

fn text_mode(mode: &str) {
    print!("\x1B[{}m", mode);
}

fn clear_console() {
    print!("\u{001b}c");
}

fn clear_line() {
    print!("\x1B[J");
}

fn hide_cursor() {
    print!("\x1B[?25l");
}

fn print_usage_and_quit() {
    eprintln!("Usage: timer.exe <duration>    or    timer.exe <duration>+<reminder_duration>");
    exit(1);
}
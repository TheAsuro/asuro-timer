extern crate win32console;
extern crate winrt_notification;

use std::{process::exit, time::Duration};

use winrt_notification::Toast;

const TM_BOLD: &'static str = "1";
const TM_DISABLE_BOLD: &'static str = "22";
const TM_BLACK: &'static str = "30";
const TM_RED: &'static str = "31";
const TM_BLUE: &'static str = "34";
const TM_DEFAULT_COLOR: &'static str = "39";

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
    
    print_remaining(remaining_duration, duration);
    while remaining_duration > 1. {
        std::thread::sleep(Duration::from_secs(60));
        remaining_duration -= 1.;
        print_remaining(remaining_duration, duration);
    }

    std::thread::sleep(Duration::from_secs_f32(remaining_duration * 60.));
    
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
    clear_console();
    set_cursor_position(1, 1);
    let console_width = win32console::console::WinConsole::output().get_screen_buffer_info().unwrap().screen_buffer_size.x;
    print_loading_bar(remaining_duration, total_duration, console_width as u32);
    text_mode(TM_BOLD);
    text_mode(TM_BLUE);
    print!("{}", remaining_duration);
    text_mode(TM_DEFAULT_COLOR);
    text_mode(TM_DISABLE_BOLD);
    println!(" minutes remaining");
}

fn print_loading_bar(progress: f32, total: f32, width: u32) {
    text_mode(TM_BOLD);
    text_mode(TM_BLACK);
    print!("[");
    let inner_bar_width = width - 2;
    let filled_steps = ((progress / total) * inner_bar_width as f32) as u32;
    for _ in 0..filled_steps {
        print!("=");
    }
    for _ in 0..(inner_bar_width - filled_steps) {
        print!(" ");
    }
    println!("]");
    text_mode(TM_DEFAULT_COLOR);
    text_mode(TM_DISABLE_BOLD);
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

fn print_usage_and_quit() {
    eprintln!("Usage: timer.exe <duration>    or    timer.exe <duration>+<reminder_duration>");
    exit(1);
}
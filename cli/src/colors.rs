//! Instagram brand colors and CLI theming
//!
//! Instagram's gradient: Purple → Pink → Orange → Yellow
//! This module provides color constants and helper functions for consistent styling.

#![allow(dead_code)]

use colored::{ColoredString, Colorize};

/// Instagram Brand Colors (RGB values)
pub mod instagram {
    // Primary gradient colors
    pub const PURPLE: (u8, u8, u8) = (131, 58, 180);    // #833AB4
    pub const PINK: (u8, u8, u8) = (225, 48, 108);      // #E1306C
    pub const ORANGE: (u8, u8, u8) = (247, 119, 55);    // #F77737
    pub const YELLOW: (u8, u8, u8) = (252, 175, 69);    // #FCAF45

    // Additional brand colors
    pub const BLUE: (u8, u8, u8) = (64, 93, 230);       // #405DE6
    pub const LIGHT_BLUE: (u8, u8, u8) = (88, 81, 219); // #5851DB
    pub const RED: (u8, u8, u8) = (237, 73, 86);        // #ED4956

    // Neutral colors
    pub const WHITE: (u8, u8, u8) = (255, 255, 255);
    pub const LIGHT_GRAY: (u8, u8, u8) = (142, 142, 142);
    pub const DARK_GRAY: (u8, u8, u8) = (38, 38, 38);
}

/// Color theme for CLI elements
pub struct Theme;

impl Theme {
    /// Apply Instagram purple to text
    pub fn purple(text: &str) -> ColoredString {
        let (r, g, b) = instagram::PURPLE;
        text.truecolor(r, g, b)
    }

    /// Apply Instagram pink to text
    pub fn pink(text: &str) -> ColoredString {
        let (r, g, b) = instagram::PINK;
        text.truecolor(r, g, b)
    }

    /// Apply Instagram orange to text
    pub fn orange(text: &str) -> ColoredString {
        let (r, g, b) = instagram::ORANGE;
        text.truecolor(r, g, b)
    }

    /// Apply Instagram yellow to text
    pub fn yellow(text: &str) -> ColoredString {
        let (r, g, b) = instagram::YELLOW;
        text.truecolor(r, g, b)
    }

    /// Apply Instagram blue to text
    pub fn blue(text: &str) -> ColoredString {
        let (r, g, b) = instagram::BLUE;
        text.truecolor(r, g, b)
    }

    /// Apply Instagram red (for errors/alerts)
    pub fn red(text: &str) -> ColoredString {
        let (r, g, b) = instagram::RED;
        text.truecolor(r, g, b)
    }

    /// Dimmed/muted text
    pub fn muted(text: &str) -> ColoredString {
        let (r, g, b) = instagram::LIGHT_GRAY;
        text.truecolor(r, g, b)
    }

    // === Semantic Colors (use these for consistent styling) ===

    /// Success messages
    pub fn success(text: &str) -> ColoredString {
        text.truecolor(46, 204, 113).bold() // Green
    }

    /// Error messages
    pub fn error(text: &str) -> ColoredString {
        let (r, g, b) = instagram::RED;
        text.truecolor(r, g, b).bold()
    }

    /// Warning messages
    pub fn warning(text: &str) -> ColoredString {
        let (r, g, b) = instagram::YELLOW;
        text.truecolor(r, g, b)
    }

    /// Usernames (@mentions)
    pub fn username(text: &str) -> ColoredString {
        let (r, g, b) = instagram::PINK;
        text.truecolor(r, g, b).bold()
    }

    /// Headers and titles
    pub fn header(text: &str) -> ColoredString {
        let (r, g, b) = instagram::PURPLE;
        text.truecolor(r, g, b).bold()
    }

    /// Accent/highlight color
    pub fn accent(text: &str) -> ColoredString {
        let (r, g, b) = instagram::ORANGE;
        text.truecolor(r, g, b)
    }

    /// Unread indicator
    pub fn unread(text: &str) -> ColoredString {
        let (r, g, b) = instagram::BLUE;
        text.truecolor(r, g, b).bold()
    }

    /// Timestamps
    pub fn timestamp(text: &str) -> ColoredString {
        let (r, g, b) = instagram::LIGHT_GRAY;
        text.truecolor(r, g, b)
    }

    /// Separator lines
    pub fn separator(width: usize) -> ColoredString {
        let (r, g, b) = instagram::LIGHT_GRAY;
        "━".repeat(width).truecolor(r, g, b)
    }

    /// Check mark (success indicator)
    pub fn check() -> ColoredString {
        "✓".truecolor(46, 204, 113).bold()
    }

    /// X mark (error indicator)
    pub fn cross() -> ColoredString {
        let (r, g, b) = instagram::RED;
        "✗".truecolor(r, g, b).bold()
    }

    /// Warning indicator
    pub fn warn_icon() -> ColoredString {
        let (r, g, b) = instagram::YELLOW;
        "⚠".truecolor(r, g, b).bold()
    }

    /// Unread dot indicator
    pub fn unread_dot() -> ColoredString {
        let (r, g, b) = instagram::BLUE;
        "●".truecolor(r, g, b)
    }
}

/// Print the Instagram-gradient banner
pub fn print_gradient_banner() {
    // Each line gets a different color from the gradient
    let lines = [
        ("    ╔══════════════════════════════════════════╗", instagram::PURPLE),
        ("    ║                                          ║", instagram::PURPLE),
        ("    ║   ▀█▀ █▀▀   █▀▄ █▀█▀█   █▀▀ █   ▀█▀      ║", instagram::PINK),
        ("    ║    █  █ █   █ █ █ ▀ █   █   █    █       ║", instagram::PINK),
        ("    ║   ▀▀▀ ▀▀▀   ▀▀  ▀   ▀   ▀▀▀ ▀▀▀ ▀▀▀      ║", instagram::ORANGE),
        ("    ║                                          ║", instagram::ORANGE),
        ("    ║       Instagram Direct Messages          ║", instagram::YELLOW),
        ("    ║            from your terminal            ║", instagram::YELLOW),
        ("    ║                                          ║", instagram::ORANGE),
        ("    ╚══════════════════════════════════════════╝", instagram::PURPLE),
    ];

    println!();
    for (line, (r, g, b)) in lines {
        println!("{}", line.truecolor(r, g, b));
    }
    println!();
}

/// Print a gradient text effect (horizontal)
pub fn gradient_text(text: &str) -> String {
    let colors = [
        instagram::PURPLE,
        instagram::PINK,
        instagram::ORANGE,
        instagram::YELLOW,
    ];

    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    if len == 0 {
        return String::new();
    }

    let mut result = String::new();
    for (i, ch) in chars.iter().enumerate() {
        // Calculate which color to use based on position
        let pos = (i as f32 / len as f32) * (colors.len() - 1) as f32;
        let idx = pos.floor() as usize;
        let next_idx = (idx + 1).min(colors.len() - 1);
        let t = pos - idx as f32;

        // Interpolate between colors
        let (r1, g1, b1) = colors[idx];
        let (r2, g2, b2) = colors[next_idx];

        let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
        let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
        let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;

        result.push_str(&format!("\x1b[38;2;{};{};{}m{}", r, g, b, ch));
    }
    result.push_str("\x1b[0m"); // Reset
    result
}

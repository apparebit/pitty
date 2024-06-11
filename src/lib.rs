//! # Pretty 🌸 Pretty
//!
//! This library brings 2020s color science to 1970s terminals to help build
//! awesome looking and adaptable terminal user interfaces. It supports
//! high-resolution colors, accurate conversion between color spaces, finding
//! the closest matching color, gamut testing and mapping, and computing text
//! contrast.
//!
//!
//! ## 1. High-Resolution Colors
//!
//! High-resolution colors from the 2020s have floating point coordinates and
//! explicit color spaces:
//!
//!   * [`ColorSpace`] enumerates supported color spaces.
//!   * [`Color`] adds `f64` coordinates to precisely represent colors.
//!
//! The example below instantiates a color in the polar Oklch color space. It
//! then converts the color to Display P3 and tests whether it is in gamut—it
//! is. Next, it converts the color sRGB and tests whether it is in gamut—it is
//! not. Finally, it maps the color into sRGB's gamut. If you are reading this
//! on a wide-gamut screen, the color swatch below the code should show two
//! distinct shades of pink, with the left one considerably more intense.
//!
//! ```
//! # use prettypretty::{Color, ColorSpace};
//! let oklch = Color::oklch(0.716, 0.349, 335);
//! let p3 = oklch.to(ColorSpace::DisplayP3);
//! assert!(p3.in_gamut());
//!
//! let not_srgb = oklch.to(ColorSpace::Srgb);
//! assert!(!not_srgb.in_gamut());
//!
//! let srgb = not_srgb.map_to_gamut();
//! assert_eq!(srgb, Color::srgb(1, 0.15942348587138203, 0.9222706101768445));
//! ```
//! <style>
//! .color-swatch {
//!     display: flex;
//! }
//! .color-swatch > div {
//!     height: 4em;
//!     width: 4em;
//!     border: black 0.5pt solid;
//!     display: flex;
//!     align-items: center;
//!     justify-content: center;
//! }
//! .small.color-swatch > div {
//!     height: 1em;
//!     width: 1em;
//! }
//! </style>
//! <div class=color-swatch>
//! <div style="background-color: oklch(0.716 0.349 335);"></div>
//! <div style="background-color: color(srgb 1 0.15942 0.92227);"></div>
//! </div>
//!
//! ### Different Color Spaces for Different Tasks
//!
//! Instead of creating a color out of nothing, we could as easily modify an
//! existing color, for example, by pushing lightness, reducing chroma, or
//! nudging the hue. As it turns out, the perceptually uniform polar coordinates
//! of Oklch and Oklrch make them great color spaces for modifying colors.
//!
//! If we need to compare colors, however, then the Cartesian coordinates of
//! Oklab and Oklrab support a straight-forward Euclidian distance metric.
//!
//! Alas, it's back to sRGB for checking that thusly manipulated colors can
//! actually be displayed in terminals. If we are targeting other platforms,
//! such as the web, then Display P3 becomes an option, too.
//!
//!
//! ## 2. Terminal Colors
//!
//! In contrast to high-resolution colors, terminal color formats from the 1970s
//! and 1980s may not even have coordinates, only an integer index. They are
//! represented through the following abstractions:
//!
//!   * [`EightBitColor`] combines [`AnsiColor`], [`EmbeddedRgb`], and
//!     [`GrayGradient`].
//!   * [`TrueColor`] represents 24-bit RGB colors, originally in the "device
//!     RGB" color space, nowadays sRGB.
//!
//! [`AnsiColor`] represents the 16 extended ANSI colors. They are eight base
//! colors—black, red, green, yellow, blue, magenta, cyan, and white—and their
//! bright variations—including bright black and bright white. ANSI colors have
//! names but no agreed-upon color values.
//!
//! [`EmbeddedRgb`] is a 6x6x6 RGB cube, i.e., every coordinate ranges from 0 to
//! 5, inclusive. Xterm's formula for converting to 24-bit RGB colors is widely
//! accepted. The color swatch below shows all 216 colors, with blue cycling
//! every column, green increasing every six columns, and red increasing every
//! row.
//!
//! <figure>
//! <div class="small color-swatch">
//! <div style="background-color: #000000;"></div>
//! <div style="background-color: #00005f;"></div>
//! <div style="background-color: #000087;"></div>
//! <div style="background-color: #0000af;"></div>
//! <div style="background-color: #0000d7;"></div>
//! <div style="background-color: #0000ff;"></div>
//! <div style="background-color: #005f00;"></div>
//! <div style="background-color: #005f5f;"></div>
//! <div style="background-color: #005f87;"></div>
//! <div style="background-color: #005faf;"></div>
//! <div style="background-color: #005fd7;"></div>
//! <div style="background-color: #005fff;"></div>
//! <div style="background-color: #008700;"></div>
//! <div style="background-color: #00875f;"></div>
//! <div style="background-color: #008787;"></div>
//! <div style="background-color: #0087af;"></div>
//! <div style="background-color: #0087d7;"></div>
//! <div style="background-color: #0087ff;"></div>
//! <div style="background-color: #00af00;"></div>
//! <div style="background-color: #00af5f;"></div>
//! <div style="background-color: #00af87;"></div>
//! <div style="background-color: #00afaf;"></div>
//! <div style="background-color: #00afd7;"></div>
//! <div style="background-color: #00afff;"></div>
//! <div style="background-color: #00d700;"></div>
//! <div style="background-color: #00d75f;"></div>
//! <div style="background-color: #00d787;"></div>
//! <div style="background-color: #00d7af;"></div>
//! <div style="background-color: #00d7d7;"></div>
//! <div style="background-color: #00d7ff;"></div>
//! <div style="background-color: #00ff00;"></div>
//! <div style="background-color: #00ff5f;"></div>
//! <div style="background-color: #00ff87;"></div>
//! <div style="background-color: #00ffaf;"></div>
//! <div style="background-color: #00ffd7;"></div>
//! <div style="background-color: #00ffff;"></div>
//! </div>
//! <div class="small color-swatch">
//! <div style="background-color: #5f0000;"></div>
//! <div style="background-color: #5f005f;"></div>
//! <div style="background-color: #5f0087;"></div>
//! <div style="background-color: #5f00af;"></div>
//! <div style="background-color: #5f00d7;"></div>
//! <div style="background-color: #5f00ff;"></div>
//! <div style="background-color: #5f5f00;"></div>
//! <div style="background-color: #5f5f5f;"></div>
//! <div style="background-color: #5f5f87;"></div>
//! <div style="background-color: #5f5faf;"></div>
//! <div style="background-color: #5f5fd7;"></div>
//! <div style="background-color: #5f5fff;"></div>
//! <div style="background-color: #5f8700;"></div>
//! <div style="background-color: #5f875f;"></div>
//! <div style="background-color: #5f8787;"></div>
//! <div style="background-color: #5f87af;"></div>
//! <div style="background-color: #5f87d7;"></div>
//! <div style="background-color: #5f87ff;"></div>
//! <div style="background-color: #5faf00;"></div>
//! <div style="background-color: #5faf5f;"></div>
//! <div style="background-color: #5faf87;"></div>
//! <div style="background-color: #5fafaf;"></div>
//! <div style="background-color: #5fafd7;"></div>
//! <div style="background-color: #5fafff;"></div>
//! <div style="background-color: #5fd700;"></div>
//! <div style="background-color: #5fd75f;"></div>
//! <div style="background-color: #5fd787;"></div>
//! <div style="background-color: #5fd7af;"></div>
//! <div style="background-color: #5fd7d7;"></div>
//! <div style="background-color: #5fd7ff;"></div>
//! <div style="background-color: #5fff00;"></div>
//! <div style="background-color: #5fff5f;"></div>
//! <div style="background-color: #5fff87;"></div>
//! <div style="background-color: #5fffaf;"></div>
//! <div style="background-color: #5fffd7;"></div>
//! <div style="background-color: #5fffff;"></div>
//! </div>
//! <div class="small color-swatch">
//! <div style="background-color: #870000;"></div>
//! <div style="background-color: #87005f;"></div>
//! <div style="background-color: #870087;"></div>
//! <div style="background-color: #8700af;"></div>
//! <div style="background-color: #8700d7;"></div>
//! <div style="background-color: #8700ff;"></div>
//! <div style="background-color: #875f00;"></div>
//! <div style="background-color: #875f5f;"></div>
//! <div style="background-color: #875f87;"></div>
//! <div style="background-color: #875faf;"></div>
//! <div style="background-color: #875fd7;"></div>
//! <div style="background-color: #875fff;"></div>
//! <div style="background-color: #878700;"></div>
//! <div style="background-color: #87875f;"></div>
//! <div style="background-color: #878787;"></div>
//! <div style="background-color: #8787af;"></div>
//! <div style="background-color: #8787d7;"></div>
//! <div style="background-color: #8787ff;"></div>
//! <div style="background-color: #87af00;"></div>
//! <div style="background-color: #87af5f;"></div>
//! <div style="background-color: #87af87;"></div>
//! <div style="background-color: #87afaf;"></div>
//! <div style="background-color: #87afd7;"></div>
//! <div style="background-color: #87afff;"></div>
//! <div style="background-color: #87d700;"></div>
//! <div style="background-color: #87d75f;"></div>
//! <div style="background-color: #87d787;"></div>
//! <div style="background-color: #87d7af;"></div>
//! <div style="background-color: #87d7d7;"></div>
//! <div style="background-color: #87d7ff;"></div>
//! <div style="background-color: #87ff00;"></div>
//! <div style="background-color: #87ff5f;"></div>
//! <div style="background-color: #87ff87;"></div>
//! <div style="background-color: #87ffaf;"></div>
//! <div style="background-color: #87ffd7;"></div>
//! <div style="background-color: #87ffff;"></div>
//! </div>
//! <div class="small color-swatch">
//! <div style="background-color: #af0000;"></div>
//! <div style="background-color: #af005f;"></div>
//! <div style="background-color: #af0087;"></div>
//! <div style="background-color: #af00af;"></div>
//! <div style="background-color: #af00d7;"></div>
//! <div style="background-color: #af00ff;"></div>
//! <div style="background-color: #af5f00;"></div>
//! <div style="background-color: #af5f5f;"></div>
//! <div style="background-color: #af5f87;"></div>
//! <div style="background-color: #af5faf;"></div>
//! <div style="background-color: #af5fd7;"></div>
//! <div style="background-color: #af5fff;"></div>
//! <div style="background-color: #af8700;"></div>
//! <div style="background-color: #af875f;"></div>
//! <div style="background-color: #af8787;"></div>
//! <div style="background-color: #af87af;"></div>
//! <div style="background-color: #af87d7;"></div>
//! <div style="background-color: #af87ff;"></div>
//! <div style="background-color: #afaf00;"></div>
//! <div style="background-color: #afaf5f;"></div>
//! <div style="background-color: #afaf87;"></div>
//! <div style="background-color: #afafaf;"></div>
//! <div style="background-color: #afafd7;"></div>
//! <div style="background-color: #afafff;"></div>
//! <div style="background-color: #afd700;"></div>
//! <div style="background-color: #afd75f;"></div>
//! <div style="background-color: #afd787;"></div>
//! <div style="background-color: #afd7af;"></div>
//! <div style="background-color: #afd7d7;"></div>
//! <div style="background-color: #afd7ff;"></div>
//! <div style="background-color: #afff00;"></div>
//! <div style="background-color: #afff5f;"></div>
//! <div style="background-color: #afff87;"></div>
//! <div style="background-color: #afffaf;"></div>
//! <div style="background-color: #afffd7;"></div>
//! <div style="background-color: #afffff;"></div>
//! </div>
//! <div class="small color-swatch">
//! <div style="background-color: #d70000;"></div>
//! <div style="background-color: #d7005f;"></div>
//! <div style="background-color: #d70087;"></div>
//! <div style="background-color: #d700af;"></div>
//! <div style="background-color: #d700d7;"></div>
//! <div style="background-color: #d700ff;"></div>
//! <div style="background-color: #d75f00;"></div>
//! <div style="background-color: #d75f5f;"></div>
//! <div style="background-color: #d75f87;"></div>
//! <div style="background-color: #d75faf;"></div>
//! <div style="background-color: #d75fd7;"></div>
//! <div style="background-color: #d75fff;"></div>
//! <div style="background-color: #d78700;"></div>
//! <div style="background-color: #d7875f;"></div>
//! <div style="background-color: #d78787;"></div>
//! <div style="background-color: #d787af;"></div>
//! <div style="background-color: #d787d7;"></div>
//! <div style="background-color: #d787ff;"></div>
//! <div style="background-color: #d7af00;"></div>
//! <div style="background-color: #d7af5f;"></div>
//! <div style="background-color: #d7af87;"></div>
//! <div style="background-color: #d7afaf;"></div>
//! <div style="background-color: #d7afd7;"></div>
//! <div style="background-color: #d7afff;"></div>
//! <div style="background-color: #d7d700;"></div>
//! <div style="background-color: #d7d75f;"></div>
//! <div style="background-color: #d7d787;"></div>
//! <div style="background-color: #d7d7af;"></div>
//! <div style="background-color: #d7d7d7;"></div>
//! <div style="background-color: #d7d7ff;"></div>
//! <div style="background-color: #d7ff00;"></div>
//! <div style="background-color: #d7ff5f;"></div>
//! <div style="background-color: #d7ff87;"></div>
//! <div style="background-color: #d7ffaf;"></div>
//! <div style="background-color: #d7ffd7;"></div>
//! <div style="background-color: #d7ffff;"></div>
//! </div>
//! <div class="small color-swatch">
//! <div style="background-color: #ff0000;"></div>
//! <div style="background-color: #ff005f;"></div>
//! <div style="background-color: #ff0087;"></div>
//! <div style="background-color: #ff00af;"></div>
//! <div style="background-color: #ff00d7;"></div>
//! <div style="background-color: #ff00ff;"></div>
//! <div style="background-color: #ff5f00;"></div>
//! <div style="background-color: #ff5f5f;"></div>
//! <div style="background-color: #ff5f87;"></div>
//! <div style="background-color: #ff5faf;"></div>
//! <div style="background-color: #ff5fd7;"></div>
//! <div style="background-color: #ff5fff;"></div>
//! <div style="background-color: #ff8700;"></div>
//! <div style="background-color: #ff875f;"></div>
//! <div style="background-color: #ff8787;"></div>
//! <div style="background-color: #ff87af;"></div>
//! <div style="background-color: #ff87d7;"></div>
//! <div style="background-color: #ff87ff;"></div>
//! <div style="background-color: #ffaf00;"></div>
//! <div style="background-color: #ffaf5f;"></div>
//! <div style="background-color: #ffaf87;"></div>
//! <div style="background-color: #ffafaf;"></div>
//! <div style="background-color: #ffafd7;"></div>
//! <div style="background-color: #ffafff;"></div>
//! <div style="background-color: #ffd700;"></div>
//! <div style="background-color: #ffd75f;"></div>
//! <div style="background-color: #ffd787;"></div>
//! <div style="background-color: #ffd7af;"></div>
//! <div style="background-color: #ffd7d7;"></div>
//! <div style="background-color: #ffd7ff;"></div>
//! <div style="background-color: #ffff00;"></div>
//! <div style="background-color: #ffff5f;"></div>
//! <div style="background-color: #ffff87;"></div>
//! <div style="background-color: #ffffaf;"></div>
//! <div style="background-color: #ffffd7;"></div>
//! <div style="background-color: #ffffff;"></div>
//! </div>
//! </figure>
//!
//! [`GrayGradient`] represents a 24-step gradient from almost black to almost
//! white. As for the embedded RGB cube, Xterm's formula for converting to
//! 24-bit RGB colors is widely accepted. The color swatch below illustrates the
//! gray gradient.
//!
//! <figure>
//! <div class="small color-swatch">
//! <div style="background-color: #121212;"></div>
//! <div style="background-color: #1c1c1c;"></div>
//! <div style="background-color: #262626;"></div>
//! <div style="background-color: #303030;"></div>
//! <div style="background-color: #3a3a3a;"></div>
//! <div style="background-color: #444444;"></div>
//! <div style="background-color: #4e4e4e;"></div>
//! <div style="background-color: #585858;"></div>
//! <div style="background-color: #626262;"></div>
//! <div style="background-color: #6c6c6c;"></div>
//! <div style="background-color: #767676;"></div>
//! <div style="background-color: #808080;"></div>
//! <div style="background-color: #8a8a8a;"></div>
//! <div style="background-color: #949494;"></div>
//! <div style="background-color: #9e9e9e;"></div>
//! <div style="background-color: #a8a8a8;"></div>
//! <div style="background-color: #b2b2b2;"></div>
//! <div style="background-color: #bcbcbc;"></div>
//! <div style="background-color: #c6c6c6;"></div>
//! <div style="background-color: #d0d0d0;"></div>
//! <div style="background-color: #dadada;"></div>
//! <div style="background-color: #e4e4e4;"></div>
//! <div style="background-color: #eeeeee;"></div>
//! <div style="background-color: #f8f8f8;"></div>
//! </div>
//! </figure>
//!
//! By combining ANSI, embedded RGB, and gray gradient colors, [`EightBitColor`]
//! covers the entire 8-bit code space. As a result, conversion from `u8` is
//! infallible, whereas it is fallible for the three component colors.
//!
//! [`TrueColor`] was a misnomer even when 24-bit video cards first came out.
//! Nowadays, the ready availability of wide-gamut and high-dynamic-range (HDR)
//! displays only underlines that true color is anything but true. But it *is*
//! the historically accurate term and lives on in this crate thanks to a mix of
//! ironic detachment and nostalgia.
//!
//! The example code below illustrates how [`AnsiColor`], [`EmbeddedRgb`],
//! [`GrayGradient`], and [`EightBitColor`] abstract over the underlying 8-bit
//! index space while also providing convenient access to RGB coordinates and
//! gray levels. Embedded RGB and gray gradient colors also nicely convert to
//! true colors, but ANSI and therefore 8-bit colors do not.
//!
//! ```
//! # use prettypretty::{AnsiColor, Coordinate::*, EightBitColor, EmbeddedRgb};
//! # use prettypretty::{GrayGradient, TrueColor};
//! assert_eq!(u8::from(AnsiColor::BrightRed), 9);
//! // Is TrueColor the equivalent of #f00, #f55, #e60000, #e74856, or what?
//!
//! let purple = EmbeddedRgb::new(3, 1, 4).unwrap();
//! let index = 16 + 3 * 36 + 1 * 6 + 4 * 1;
//! assert_eq!(index, 134);
//! assert_eq!(u8::from(purple), index);
//! assert_eq!(TrueColor::from(purple), TrueColor::new(175, 95, 215));
//!
//! let gray = GrayGradient::new(18).unwrap();
//! let index = 232 + 18;
//! assert_eq!(index, 250);
//! assert_eq!(gray.level(), 18);
//! assert_eq!(u8::from(gray), index);
//! assert_eq!(TrueColor::from(gray), TrueColor::new(188, 188, 188));
//!
//! let green = EightBitColor::from(71);
//! assert!(green.is_rgb());
//! let green = green.rgb().unwrap();
//! assert_eq!(green[C1], 1);
//! assert_eq!(green[C2], 3);
//! assert_eq!(green[C3], 1);
//! assert_eq!(TrueColor::from(green), TrueColor::new(95, 175, 95));
//! ```
//! <div class=color-swatch>
//! <div style="background: repeating-linear-gradient(45deg, #fff, #fff 10px, #fcc 10px, #fcc 20px);">
//! <span style="font-weight: bold; font-size: 2em;">?</span>
//! </div>
//! <div style="background-color: #af5fd7;"></div>
//! <div style="background-color: #bcbcbc;"></div>
//! <div style="background-color: #5faf5f;"></div>
//! </div>
//!
//!
//! ## 3. Integration of High-Resolution and Terminal Colors
//!
//! To apply 2020s color science to terminal colors, we need to be able to
//! convert terminal to high-resolution colors and back again:
//!
//!   * [`Theme`] provides high-resolution color values for the 16 extended ANSI
//!     colors and terminal defaults.
//!   * [`ThemeBuilder`] helps to incrementally fill in a theme.
//!   * [`ColorMatcher`] stores high-resolution color values for all
//!     8-bit terminal colors to find closest matching color.
//!
//!
//!
//! ## 4. BYOIO: Bring Your Own (Terminal) I/O
//!
//! Unlike the Python version, the Rust version of prettypretty does not (yet?)
//! include its own facilities for styled text or terminal I/O. Instead, it is
//! designed to be a lightweight addition that focuses on color management only.
//! To use this crate, an application must create its own instances of [`Theme`]
//! and [`ColorMatcher`]. While this crate contains one default theme,
//! surprisingly called [`DEFAULT_THEME`], that theme is suitable for tests but
//! no more.
//!
//! To fill in an accurate terminal theme, the application should use the ANSI
//! escape sequences
//! ```text
//! "{OSC}{10..=11};?{ST}"
//! ```
//! and
//! ```text
//! "{OSC}4;{0..=15};?{ST}"
//! ```
//! to query the terminal for its two default and 16 extended ANSI colors. The
//! responses are ANSI escape sequences with the exact same prefix as requests,
//! *before* the question mark, followed by the color in X Windows `rgb:`
//! format, followed by ST. Once you stripped the prefix and suffix from a
//! response, you can use the `FromStr` trait to parse the X Windows color
//! format into a color object.
//!
//! As usual, OSC stands for the character sequence `\x1b]` (escape, closing
//! square bracket) and ST stands for the character sequence `\x1b\\` (escape,
//! backslash). Some terminals answer with `\x0b` (bell) instead of ST.
//!
//!
//! ## Color Swatches
//!
//! As already illustrated above, most code examples come with their own color
//! swatches, which show the color values mentioned in the code. Where possible,
//! swatches use the exact same color spaces as the code (sRGB, Display P3,
//! Oklab, or Oklch). Otherwise, they fall back on an equivalent color in a
//! comparable color space (Oklrab and Oklrch).

mod color;
mod serde;
mod term_color;
mod util;

pub use color::Color;
pub use color::ColorSpace;
pub use util::Coordinate;

pub use term_color::AnsiColor;
pub use term_color::EightBitColor;
pub use term_color::EmbeddedRgb;
pub use term_color::GrayGradient;
pub use term_color::TrueColor;

pub use serde::ColorFormatError;
pub use term_color::OutOfBoundsError;

// ====================================================================================================================
// Color Themes
// ====================================================================================================================

/// A color theme.
///
/// ANSI colors do not have intrinsic color values. However, a color theme does
/// have color values for the 16 extended ANSI colors as well as the foreground
/// and background default colors. By itself, a theme enables the conversion of
/// ANSI colors to high-resolution colors. Through a [`ColorMatcher`], a
/// theme also enables conversion of high-resolution colors ANSI (and possibly
/// 8-bit) colors.
#[derive(Clone, Debug, Default)]
pub struct Theme {
    foreground: Color,
    background: Color,
    black: Color,
    red: Color,
    green: Color,
    yellow: Color,
    blue: Color,
    magenta: Color,
    cyan: Color,
    white: Color,
    bright_black: Color,
    bright_red: Color,
    bright_green: Color,
    bright_yellow: Color,
    bright_blue: Color,
    bright_magenta: Color,
    bright_cyan: Color,
    bright_white: Color,
}

impl Theme {
    /// Instantiate a new theme builder for incrementally building a theme.
    pub fn builder() -> ThemeBuilder {
        ThemeBuilder::default()
    }

    /// Access the theme's foreground color.
    pub const fn foreground(&self) -> &Color {
        &self.foreground
    }

    /// Access the theme's background color.
    pub const fn background(&self) -> &Color {
        &self.background
    }

    // Access the theme's ANSI colors.
    pub const fn ansi(&self, value: AnsiColor) -> &Color {
        use AnsiColor::*;

        match value {
            Black => &self.black,
            Red => &self.red,
            Green => &self.green,
            Yellow => &self.yellow,
            Blue => &self.blue,
            Magenta => &self.magenta,
            Cyan => &self.cyan,
            White => &self.white,
            BrightBlack => &self.bright_black,
            BrightRed => &self.bright_red,
            BrightGreen => &self.bright_green,
            BrightYellow => &self.bright_yellow,
            BrightBlue => &self.bright_blue,
            BrightMagenta => &self.bright_magenta,
            BrightCyan => &self.bright_cyan,
            BrightWhite => &self.bright_white,
        }
    }
}

/// The default theme.
///
/// This theme exists to demonstrate the functionality enabled by themes as well
/// as for testing. It uses the colors of [VGA text
/// mode](https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit).
pub const DEFAULT_THEME: Theme = Theme {
    foreground: Color::new(ColorSpace::Srgb, 0.0, 0.0, 0.0),
    background: Color::new(ColorSpace::Srgb, 1.0, 1.0, 1.0),
    black: Color::new(ColorSpace::Srgb, 0.0, 0.0, 0.0),
    red: Color::new(ColorSpace::Srgb, 0.666666666666667, 0.0, 0.0),
    green: Color::new(ColorSpace::Srgb, 0.0, 0.666666666666667, 0.0),
    yellow: Color::new(ColorSpace::Srgb, 0.666666666666667, 0.333333333333333, 0.0),
    blue: Color::new(ColorSpace::Srgb, 0.0, 0.0, 0.666666666666667),
    magenta: Color::new(ColorSpace::Srgb, 0.666666666666667, 0.0, 0.666666666666667),
    cyan: Color::new(ColorSpace::Srgb, 0.0, 0.666666666666667, 0.666666666666667),
    white: Color::new(ColorSpace::Srgb, 0.666666666666667, 0.666666666666667, 0.666666666666667),
    bright_black: Color::new(ColorSpace::Srgb, 0.333333333333333, 0.333333333333333, 0.333333333333333),
    bright_red: Color::new(ColorSpace::Srgb, 1.0, 0.333333333333333, 0.333333333333333),
    bright_green: Color::new(ColorSpace::Srgb, 0.333333333333333, 1.0, 0.333333333333333),
    bright_yellow: Color::new(ColorSpace::Srgb, 1.0, 1.0, 0.333333333333333),
    bright_blue: Color::new(ColorSpace::Srgb, 0.333333333333333, 0.333333333333333, 1.0),
    bright_magenta: Color::new(ColorSpace::Srgb, 1.0, 0.333333333333333, 1.0),
    bright_cyan: Color::new(ColorSpace::Srgb, 0.333333333333333, 1.0, 1.0),
    bright_white: Color::new(ColorSpace::Srgb, 1.0, 1.0, 1.0),
};

/// An incremental theme builder.
#[derive(Clone, Debug, Default)]
pub struct ThemeBuilder {
    theme: Theme,
}

impl ThemeBuilder {
    /// Update the default foreground color.
    pub fn foreground(&mut self, value: Color) -> &mut Self {
        self.theme.foreground = value;
        self
    }

    /// Update the default background color.
    pub fn background(&mut self, value: Color) -> &mut Self {
        self.theme.background = value;
        self
    }

    /// Update the color value for an ANSI color.
    pub fn with_ansi_color(&mut self, term: AnsiColor, value: Color) -> &mut Self {
        use AnsiColor::*;

        match term {
            Black => self.theme.black = value,
            Red => self.theme.red = value,
            Green => self.theme.green = value,
            Yellow => self.theme.yellow = value,
            Blue => self.theme.blue = value,
            Magenta => self.theme.magenta = value,
            Cyan => self.theme.cyan = value,
            White => self.theme.white = value,
            BrightBlack => self.theme.bright_black = value,
            BrightRed => self.theme.bright_red = value,
            BrightGreen => self.theme.bright_green = value,
            BrightYellow => self.theme.bright_yellow = value,
            BrightBlue => self.theme.bright_blue = value,
            BrightMagenta => self.theme.bright_magenta = value,
            BrightCyan => self.theme.bright_cyan = value,
            BrightWhite => self.theme.bright_white = value,
        }

        self
    }

    /// Determine whether this theme builder is ready, i.e., all fields have
    /// some color value.
    fn ready(&self) -> bool {
        !self.theme.foreground.is_default()
            && !self.theme.background.is_default()
            // Skip black
            && !self.theme.red.is_default()
            && !self.theme.green.is_default()
            && !self.theme.yellow.is_default()
            && !self.theme.blue.is_default()
            && !self.theme.magenta.is_default()
            && !self.theme.cyan.is_default()
            && !self.theme.white.is_default()
            && !self.theme.bright_black.is_default()
            && !self.theme.bright_red.is_default()
            && !self.theme.bright_green.is_default()
            && !self.theme.bright_yellow.is_default()
            && !self.theme.bright_blue.is_default()
            && !self.theme.bright_magenta.is_default()
            && !self.theme.bright_cyan.is_default()
            && !self.theme.bright_white.is_default()
    }

    /// Build the theme. If all colors of the theme but black have been updated,
    /// this method returns a clone of the current theme.
    pub fn build(self) -> Option<Theme> {
        if !self.ready() {
            None
        } else {
            Some(self.theme.clone())
        }
    }
}

// https://stackoverflow.com/questions/74085531/alternative-to-static-mut-and-unsafe-while-managing-global-application-state

// --------------------------------------------------------------------------------------------------------------------

impl From<TrueColor> for Color {
    /// Convert the "true" color object into a *true* color object... 🤪
    fn from(value: TrueColor) -> Color {
        let [r, g, b] = *value.coordinates();
        Color::srgb(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }
}

impl From<EmbeddedRgb> for Color {
    /// Instantiate a new color from the embedded RGB value.
    fn from(value: EmbeddedRgb) -> Color {
        TrueColor::from(value).into()
    }
}

impl From<GrayGradient> for Color {
    /// Instantiate a new color from the embedded RGB value.
    fn from(value: GrayGradient) -> Color {
        TrueColor::from(value).into()
    }
}

// ====================================================================================================================
// Color Matcher
// ====================================================================================================================

/// A state container for matching terminal colors.
///
/// A color matcher owns the 256 color objects necessary for high-quality
/// conversion from arbitrary instances of [`Color`] to 8-bit or ANSI colors.
/// Conversion to 8-bit colors does *not* consider the 16 extended ANSI colors
/// as candidates because they become highly visible outliers when matching
/// several graduated colors.
///
/// Every color matcher instance incorporates the colors from the theme passed
/// to its constructor. Hence, if the theme changes, so should the color
/// matcher.
///
/// <style>
/// .color-swatch {
///     display: flex;
/// }
/// .color-swatch > div {
///     height: 4em;
///     width: 4em;
///     border: black 0.5pt solid;
///     display: flex;
///     align-items: center;
///     justify-content: center;
/// }
/// </style>
#[derive(Debug)]
pub struct ColorMatcher {
    ansi: Vec<Color>,
    eight_bit: Vec<Color>,
}

impl ColorMatcher {
    /// Create a new terminal color matcher. This method initializes the
    /// internal state, which comprises 256 color objects, 16 for the ANSI
    /// colors (based on the theme), 216 for the embedded RGB colors, and 24 for
    /// the gray gradient colors.
    pub fn new(theme: &Theme) -> Self {
        let ansi = (0..=15)
            .map(|n| {
                theme
                    .ansi(AnsiColor::try_from(n).unwrap())
                    .to(ColorSpace::Oklrab)
            })
            .collect();

        let eight_bit: Vec<Color> = (16..=231)
            .map(|n| Color::from(EmbeddedRgb::try_from(n).unwrap()).to(ColorSpace::Oklrab))
            .chain(
                (232..=255)
                    .map(|n| Color::from(GrayGradient::try_from(n).unwrap()).to(ColorSpace::Oklrab)),
            )
            .collect();

        Self { ansi, eight_bit }
    }

    /// Find the ANSI color that comes closest to the given color.
    ///
    ///
    /// # Example
    ///
    /// The example code below matches `#ffa563` and `#ff9600` to ANSI colors
    /// under the default theme. The first color matches ANSI cyan, which is a
    /// very poor fit and demonstrates that even high-resolution, perceptually
    /// uniform colors cannot make up for the extremely limited choices. It also
    /// suggests that, maybe, finding matches in polar coordinates may be
    /// preferable for ANSI colors, since it can prioritize hues over chroma.
    ///
    /// ```
    /// # use prettypretty::{Color, ColorFormatError, ColorMatcher, ColorSpace};
    /// # use prettypretty::{DEFAULT_THEME};
    /// # use std::str::FromStr;
    /// let matcher = ColorMatcher::new(&DEFAULT_THEME);
    ///
    /// let color = Color::from_str("#ffa563")?;
    /// let ansi = matcher.to_ansi(&color);
    /// assert_eq!(u8::from(ansi), 7);
    ///
    /// let color = Color::from_str("#ff9600")?;
    /// let ansi = matcher.to_ansi(&color);
    /// assert_eq!(u8::from(ansi), 9);
    /// # Ok::<(), ColorFormatError>(())
    /// ```
    /// <div class=color-swatch>
    /// <div style="background-color: #ffa563;"></div>
    /// <div style="background-color: #00aaaa;"></div>
    /// <div style="background-color: #ff9600;"></div>
    /// <div style="background-color: #ff5555;"></div>
    /// </div>
    pub fn to_ansi(&self, color: &Color) -> AnsiColor {
        // SAFETY: self.ansi holds 16 elements, hence closest() returns index 0..=15.
        color
            .closest(&self.ansi)
            .map(|idx| AnsiColor::try_from(idx as u8))
            .unwrap()
            .unwrap()
    }

    /// Find the 8-bit color that comes closest to the given color.
    ///
    ///
    /// # Example
    ///
    /// The example below converts every color of the RGB cube embedded in 8-bit
    /// colors to a high-resolution color in sRGB, which is validated by the
    /// first two assertions, and then uses a color matcher to convert that
    /// color back to an embedded RGB color. The result is the original color,
    /// now wrapped as an 8-bit color, which is validated by the third
    /// assertion. The example demonstrates that the 216 colors in the embedded
    /// RGB cube still are closest to themselves after conversion to Oklrch.
    ///
    /// ```
    /// # use prettypretty::{Color, ColorSpace, DEFAULT_THEME, EightBitColor};
    /// # use prettypretty::{EmbeddedRgb, OutOfBoundsError, ColorMatcher};
    /// # use prettypretty::Coordinate::C1;
    /// let matcher = ColorMatcher::new(&DEFAULT_THEME);
    ///
    /// for r in 0..5 {
    ///     for g in 0..5 {
    ///         for b in 0..5 {
    ///             let embedded = EmbeddedRgb::new(r, g, b)?;
    ///             let color = Color::from(embedded);
    ///             assert_eq!(color.space(), ColorSpace::Srgb);
    ///
    ///             let c1 = if r == 0 {
    ///                 0.0
    ///             } else {
    ///                 (55.0 + 40.0 * (r as f64)) / 255.0
    ///             };
    ///             assert!((color[C1] - c1).abs() < f64::EPSILON);
    ///
    ///             let result = matcher.to_eight_bit(&color);
    ///             assert_eq!(result, EightBitColor::Rgb(embedded));
    ///         }
    ///     }
    /// }
    /// # Ok::<(), OutOfBoundsError>(())
    /// ```
    pub fn to_eight_bit(&self, color: &Color) -> EightBitColor {
        // SAFETY: self.eight_bit holds 240 elements, hence closest() returns
        // index 0..=239, which becomes 16..=255 after addition.
        color
            .closest(&self.eight_bit)
            .map(|idx| EightBitColor::from(idx as u8 + 16))
            .unwrap()
    }
}

// ====================================================================================================================

#[cfg(test)]
mod test {
    use super::{AnsiColor, Color, DEFAULT_THEME, OutOfBoundsError, ColorMatcher};

    #[test]
    fn test_matcher() -> Result<(), OutOfBoundsError> {
        let matcher = ColorMatcher::new(&DEFAULT_THEME);

        let result = matcher.to_ansi(&Color::srgb(1.0, 1.0, 0.0));
        assert_eq!(result, AnsiColor::BrightYellow);

        Ok(())
    }
}

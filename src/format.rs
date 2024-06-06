//! # Terminal Color Formats
//!
//! This module provides the abstractions for terminal color formats. Unlike the
//! more general and precise color abstraction, this module is informed by the
//! many restrictions of terminals. One key consequence is that even colors with
//! three coordinates do not use floating point but integral numbers drawn from
//! a specific range.

#![allow(dead_code)]

// ====================================================================================================================
// Errors
// ====================================================================================================================

use std::ops::RangeInclusive;

/// An out-of-bounds error.
///
/// Trying to convert an invalid byte value to a terminal color results in an
/// out-of-bounds error. It combines the invalid value with the expected range
/// of values. The following ranges occur in practice:
///
///   * `0..=5` for individual coordinates of the embedded RGB cube;
///   * `0..=15` for the 16 extended ANSI colors;
///   * `16..=215` for the 8-bit values of the embedded RGB cube;
///   * `232..=255` for the 24-step gray gradient.
#[derive(Clone, Debug)]
pub struct OutOfBoundsError {
    value: u8,
    expected: RangeInclusive<u8>,
}

impl OutOfBoundsError {
    /// Access the offending value.
    pub fn value(&self) -> u8 {
        self.value
    }

    /// Access the expected range.
    pub fn expected(&self) -> &RangeInclusive<u8> {
        &self.expected
    }
}

impl std::fmt::Display for OutOfBoundsError {
    /// Format this out-of-bounds error.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} should fit into range {}..={}",
            self.value, self.expected.start(), self.expected.end()
        )
    }
}

impl std::error::Error for OutOfBoundsError {}


// ====================================================================================================================
// Ansi Color
// ====================================================================================================================

/// The 16 extended ANSI colors.
///
/// Despite their names, *white* and *bright black* are obviously distinct from
/// white and black, respectively. Both are gray, with *white* closer to *bright
/// white* than either black and *bright black* closer to *black* than either
/// white. In other words, the 16 extended ANSI colors include a four-color gray
/// gradient from *black* to *bright black* to *white* to *bright white*.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl TryFrom<u8> for AnsiColor {
    type Error = OutOfBoundsError;

    /// Try to convert the unsigned byte to an ANSI color.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => AnsiColor::Black,
            1 => AnsiColor::Red,
            2 => AnsiColor::Green,
            3 => AnsiColor::Yellow,
            4 => AnsiColor::Blue,
            5 => AnsiColor::Magenta,
            6 => AnsiColor::Cyan,
            7 => AnsiColor::White,
            8 => AnsiColor::BrightBlack,
            9 => AnsiColor::BrightRed,
            10 => AnsiColor::BrightGreen,
            11 => AnsiColor::BrightYellow,
            12 => AnsiColor::BrightBlue,
            13 => AnsiColor::BrightMagenta,
            14 => AnsiColor::BrightCyan,
            15 => AnsiColor::BrightWhite,
            _ => {
                return Err(OutOfBoundsError { value, expected: 0..=15 })
            }
        })
    }
}

impl From<AnsiColor> for u8 {
    /// Convert an ANSI color to an unsigned byte.
    fn from(value: AnsiColor) -> u8 {
        value as u8
    }
}

// ====================================================================================================================
// The Embedded 6x6x6 RGB
// ====================================================================================================================

/// The 6x6x6 RGB cube embedded in 8-bit terminal colors.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EmbeddedRgb([u8; 3]);

impl EmbeddedRgb {
    /// Instantiate a new embedded RGB value from its coordinates.
    pub fn new(r: u8, g: u8, b: u8) -> Result<Self, OutOfBoundsError> {
        if r >= 6 {
            Err(OutOfBoundsError { value: r, expected: 0..=5 })
        } else if g >= 6 {
            Err(OutOfBoundsError { value: g, expected: 0..=5 })
        } else if b >= 6 {
            Err(OutOfBoundsError { value: b, expected: 0..=5 })
        } else {
            Ok(Self([r, g, b]))
        }
    }

    /// Access the coordinates of the embedded RGB color.
    #[inline]
    pub fn coordinates(&self) -> &[u8; 3] {
        &self.0
    }
}

impl TryFrom<u8> for EmbeddedRgb {
    type Error = OutOfBoundsError;

    /// Try instantiating an embedded RGB color from an unsigned byte.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 16 || value >= 231 {
            Err(Self::Error { value, expected: 16..=231 })
        } else {
            let mut b = value - 16;
            let r = b / 36;
            b -= r * 36;
            let g = b / 6;
            b -= g * 6;

            Ok(Self([r, g, b]))
        }
    }
}

impl From<EmbeddedRgb> for u8 {
    /// Convert an embedded RGB color to an unsigned byte.
    fn from(value: EmbeddedRgb) -> u8 {
        let [r, g, b] = value.0;
        16 + 36 * r + 6 * g + b
    }
}

// ====================================================================================================================
// Gray Gradient
// ====================================================================================================================

/// The 24-step gray gradient embedded in 8-bit terminal colors.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GrayGradient(u8);

impl GrayGradient {
    /// Instantiate a new gray gradient from the level value. This associated
    /// function differs from `try_from()` in the accepted range: `new()`
    /// implies intentional instantiation and hence accepts `0..=23`, whereas
    /// `try_from()` implies conversion from unsigned bytes, i.e., 8-bit color,
    /// and hence accepts `232..=255`.
    pub fn new(value: u8) -> Result<Self, OutOfBoundsError> {
        if value <= 23 {
            Ok(Self(value))
        } else {
            Err(OutOfBoundsError { value, expected: 0..=23 })
        }
    }

    /// Access the gray level `0..24`.
    pub fn level(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for GrayGradient {
    type Error = OutOfBoundsError;

    /// Try instantiating a gray gradient value from an unsigned byte, that is,
    /// the numerical representation of 8-bit color. In contrast to `new()`,
    /// this associated function accepts 232..=255.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if 232 <= value {
            Ok(Self(value - 232))
        } else {
            Err(OutOfBoundsError { value, expected: 232..=255 })
        }
    }
}

impl From<GrayGradient> for u8 {
    /// Convert the gray gradient to an unsigned byte.
    fn from(value: GrayGradient) -> u8 {
        232 + value.0
    }
}

// ====================================================================================================================
// 8-bit Color
// ====================================================================================================================

/// An 8-bit terminal color.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EightBitColor {
    Ansi(AnsiColor),
    Rgb(EmbeddedRgb),
    Gray(GrayGradient),
}

impl EightBitColor {
    /// Instantiate an 8-bit color from its numerical representation.
    pub fn new(value: u8) -> Self {
        use EightBitColor::*;

        if value <= 15 {
            Ansi(value.try_into().unwrap())
        } else if value <= 215 {
            Rgb(value.try_into().unwrap())
        } else {
            Gray(value.try_into().unwrap())
        }
    }

    /// Determine whether this 8-bit color is an ANSI color.
    pub fn is_ansi(&self) -> bool {
        if let Self::Ansi(_) = *self {
            true
        } else {
            false
        }
    }

    /// Access this 8-bit color as an ANSI color.
    pub fn ansi(&self) -> Option<AnsiColor> {
        if let Self::Ansi(color) = *self {
            Some(color)
        } else {
            None
        }
    }

    /// Determine whether this 8-bit color is an embedded RGB color.
    pub fn is_rgb(&self) -> bool {
        if let Self::Rgb(_) = *self {
            true
        } else {
            false
        }
    }

    /// Access this 8-bit color as an embedded RGB color.
    pub fn rgb(&self) -> Option<EmbeddedRgb> {
        if let Self::Rgb(color) = *self {
            Some(color)
        } else {
            None
        }
    }

    /// Determine whether this 8-bit color is a gray gradient.
    pub fn is_gray(&self) -> bool {
        if let Self::Gray(_) = *self {
            true
        } else {
            false
        }
    }

    /// Access this 8-bit color as a gray gradient.
    pub fn gray(&self) -> Option<GrayGradient> {
        if let Self::Gray(color) = *self {
            Some(color)
        } else {
            None
        }
    }
}

impl From<u8> for EightBitColor {
    /// Convert an unsigned byte to an 8-bit color.
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<EightBitColor> for u8 {
    /// Convert an 8-bit color to an unsigned byte.
    fn from(value: EightBitColor) -> u8 {
        use EightBitColor::*;

        match value {
            Ansi(color) => color as u8,
            Rgb(color) => color.into(),
            Gray(color) => color.into(),
        }
    }
}

// ====================================================================================================================
// True Color (24-bit RGB)
// ====================================================================================================================

/// A true color, i.e., 24-bit color.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TrueColor([u8; 3]);

impl TrueColor {
    /// Create a new true color from its coordinates.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b])
    }

    /// Access the coordinates.
    #[inline]
    pub fn coordinates(&self) -> &[u8; 3] {
        &self.0
    }
}

impl From<EmbeddedRgb> for TrueColor {
    /// Instantiate a true color from a gray gradient value.
    fn from(value: EmbeddedRgb) -> Self {
        let [r, g, b] = value.coordinates();
        Self([55 + 40 * r, 55 + 40 * g, 55 + 40 * b])
    }
}

impl From<GrayGradient> for TrueColor {
    /// Instantiate a true color from a gray gradient value.
    fn from(value: GrayGradient) -> Self {
        let level = 8 + 10 * value.level();
        Self([level, level, level])
    }
}

// ====================================================================================================================
// Fidelity
// ====================================================================================================================

/// Terminal fidelity.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fidelity {
    /// The equivalent of true color.
    FullColor,
    /// The equivalent of 8-bit color.
    ReducedColor,
    /// The equivalent of ANSI colors.
    MinimalColor,
    /// No colors, but ANSI escape codes are fine.
    NoColor,
    /// No colors, no ANSI escape codes.
    None,
}


#[cfg(test)]
mod test {
    use crate::format::{AnsiColor, EightBitColor, GrayGradient};

    use super::{EmbeddedRgb, OutOfBoundsError};

    #[test]
    fn test_conversion() -> Result<(), OutOfBoundsError> {
        let magenta = AnsiColor::Magenta;
        assert_eq!(magenta as u8, 5);

        let green= EmbeddedRgb::new(0, 4, 0)?;
        assert_eq!(green.coordinates(), &[0, 4, 0]);

        let gray = GrayGradient::new(12)?;
        assert_eq!(gray.level(), 12);

        let also_magenta = EightBitColor::Ansi(AnsiColor::Magenta);
        let also_green = EightBitColor::Rgb(green);
        let also_gray = EightBitColor::Gray(gray);

        assert_eq!(u8::from(also_magenta), 5);
        assert_eq!(u8::from(also_green), 40);
        assert_eq!(u8::from(also_gray), 244);

        assert_eq!(EightBitColor::from(5), also_magenta);
        assert_eq!(EightBitColor::from(40), also_green);
        assert_eq!(EightBitColor::from(244), also_gray);

        Ok(())
    }
}
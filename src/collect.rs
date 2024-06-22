use crate::{
    AnsiColor, Color, ColorSpace, EightBitColor, EmbeddedRgb, GrayGradient, Layer, OkVersion,
};

// ====================================================================================================================
// Color Themes
// ====================================================================================================================

/// A color theme with concrete color values.
///
/// A color theme provides concrete color values for the foreground and
/// background defaults as well as for the 16 extended ANSI colors. They are
/// accessed (and also updated) through index expressions using [`Layer`] and
/// [`AnsiColor`].
///
/// By itself, a theme enables the conversion of ANSI colors to high-resolution
/// colors. Through a [`ColorMatcher`], a theme also enables the (lossy)
/// conversion of high-resolution colors to ANSI and 8-bit colors.
#[derive(Clone, Debug, Default)]
pub struct Theme {
    colors: [Color; 18],
}

impl Theme {
    /// Instantiate a new theme. The colors of the new theme are all the default
    /// color.
    pub fn new() -> Self {
        Theme::default()
    }
}

impl std::ops::Index<Layer> for Theme {
    type Output = Color;

    /// Access the color value for the layer's default color.
    fn index(&self, index: Layer) -> &Self::Output {
        match index {
            Layer::Foreground => &self.colors[0],
            Layer::Background => &self.colors[1],
        }
    }
}

impl std::ops::IndexMut<Layer> for Theme {
    /// Mutably access the color value for the layer's default color.
    fn index_mut(&mut self, index: Layer) -> &mut Self::Output {
        match index {
            Layer::Foreground => &mut self.colors[0],
            Layer::Background => &mut self.colors[1],
        }
    }
}

impl std::ops::Index<AnsiColor> for Theme {
    type Output = Color;

    /// Access the color value for the ANSI color.
    fn index(&self, index: AnsiColor) -> &Self::Output {
        &self.colors[2 + index as usize]
    }
}

impl std::ops::IndexMut<AnsiColor> for Theme {
    /// Mutably access the color value for the ANSI color.
    fn index_mut(&mut self, index: AnsiColor) -> &mut Self::Output {
        &mut self.colors[2 + index as usize]
    }
}

/// The default theme.
///
/// This theme exists to demonstrate the functionality enabled by themes as well
/// as for testing. It uses the colors of [VGA text
/// mode](https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit).
pub const DEFAULT_THEME: Theme = Theme {
    colors: [
        Color::new(ColorSpace::Srgb, 0.0, 0.0, 0.0),
        Color::new(ColorSpace::Srgb, 1.0, 1.0, 1.0),
        Color::new(ColorSpace::Srgb, 0.0, 0.0, 0.0),
        Color::new(ColorSpace::Srgb, 0.666666666666667, 0.0, 0.0),
        Color::new(ColorSpace::Srgb, 0.0, 0.666666666666667, 0.0),
        Color::new(ColorSpace::Srgb, 0.666666666666667, 0.333333333333333, 0.0),
        Color::new(ColorSpace::Srgb, 0.0, 0.0, 0.666666666666667),
        Color::new(ColorSpace::Srgb, 0.666666666666667, 0.0, 0.666666666666667),
        Color::new(ColorSpace::Srgb, 0.0, 0.666666666666667, 0.666666666666667),
        Color::new(
            ColorSpace::Srgb,
            0.666666666666667,
            0.666666666666667,
            0.666666666666667,
        ),
        Color::new(
            ColorSpace::Srgb,
            0.333333333333333,
            0.333333333333333,
            0.333333333333333,
        ),
        Color::new(ColorSpace::Srgb, 1.0, 0.333333333333333, 0.333333333333333),
        Color::new(ColorSpace::Srgb, 0.333333333333333, 1.0, 0.333333333333333),
        Color::new(ColorSpace::Srgb, 1.0, 1.0, 0.333333333333333),
        Color::new(ColorSpace::Srgb, 0.333333333333333, 0.333333333333333, 1.0),
        Color::new(ColorSpace::Srgb, 1.0, 0.333333333333333, 1.0),
        Color::new(ColorSpace::Srgb, 0.333333333333333, 1.0, 1.0),
        Color::new(ColorSpace::Srgb, 1.0, 1.0, 1.0),
    ],
};

// ====================================================================================================================
// Color Matcher
// ====================================================================================================================

/// Conversion from high-resolution to terminal colors by exhaustive search
///
/// A color matcher converts [`Color`] objects to [`EightBitColor`] or
/// [`AnsiColor`] by comparing the original color to all 8-bit colors but the
/// ANSI colors or all ANSI colors, respectively, and returning the closest
/// matching color. Conversion to 8-bit colors does not consider the ANSI colors
/// because they become highly visible outliers when converting several
/// graduated colors.
///
/// To be meaningful, the search for the closest color is performed in a
/// perceptually uniform color space and uses high-resolution colors that are
/// equivalent to the 8-bit terminal colors, including the ANSI color values
/// provided by a [`Theme`]. This struct computes all necessary color
/// coordinates upon creation and caches them for its lifetime, which maximizes
/// the performance of conversion.
///
/// Since a color matcher incorporates the color values from a [`Theme`], an
/// application should regenerate its color matcher if the current theme
/// changes.
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
    space: ColorSpace,
    ansi: Vec<[f64; 3]>,
    eight_bit: Vec<[f64; 3]>,
}

impl ColorMatcher {
    /// Create a new terminal color matcher. This method initializes the
    /// matcher's internal state, which comprises the coordinates for the 256
    /// 8-bit colors, 16 for the ANSI colors based on the provided theme, 216
    /// for the embedded RGB colors, and 24 for the gray gradient, all in the
    /// requested color space.
    pub fn new(theme: &Theme, ok_version: OkVersion) -> Self {
        let space = ok_version.cartesian_space();
        let ansi = (0..=15)
            .map(|n| {
                *theme[AnsiColor::try_from(n).unwrap()]
                    .to(space)
                    .coordinates()
            })
            .collect();
        let eight_bit: Vec<[f64; 3]> = (16..=231)
            .map(|n| {
                *Color::from(EmbeddedRgb::try_from(n).unwrap())
                    .to(space)
                    .coordinates()
            })
            .chain((232..=255).map(|n| {
                *Color::from(GrayGradient::try_from(n).unwrap())
                    .to(space)
                    .coordinates()
            }))
            .collect();

        Self {
            space,
            ansi,
            eight_bit,
        }
    }

    /// Find the ANSI color that comes closest to the given color.
    ///
    ///
    /// # Examples
    ///
    /// The example code below matches shades of orange `#ffa563` and `#ff9600`
    /// to ANSI colors under the default theme in both Oklab and Oklrab. In both
    /// versions of the color space, the first orange consistently matches ANSI
    /// white and the second orange bright red.
    ///
    /// ```
    /// # use prettypretty::{Color, ColorFormatError, ColorMatcher, ColorSpace};
    /// # use prettypretty::{DEFAULT_THEME, OkVersion};
    /// # use std::str::FromStr;
    /// let matcher = ColorMatcher::new(&DEFAULT_THEME, OkVersion::Original);
    ///
    /// let color = Color::from_str("#ffa563")?;
    /// let ansi = matcher.to_ansi(&color);
    /// assert_eq!(u8::from(ansi), 7);
    ///
    /// let color = Color::from_str("#ff9600")?;
    /// let ansi = matcher.to_ansi(&color);
    /// assert_eq!(u8::from(ansi), 9);
    /// // ---------------------------------------------------------------------
    /// let matcher = ColorMatcher::new(&DEFAULT_THEME, OkVersion::Revised);
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
    /// <div style="background-color: #aaaaaa;"></div>
    /// <div style="background-color: #ff9600;"></div>
    /// <div style="background-color: #ff5555;"></div>
    /// <div style="background-color: #ffa563;"></div>
    /// <div style="background-color: #aaaaaa;"></div>
    /// <div style="background-color: #ff9600;"></div>
    /// <div style="background-color: #ff5555;"></div>
    /// </div>
    /// <br>
    ///
    /// That `#ffa563` has white's `#aaaaaa` as its closest match is more than a
    /// little ironic: The color has almost the same hue and chroma as the
    /// default theme's yellow, which really is a dark orange or brown. The
    /// figure below illustrates just that, plotting all non-gray ANSI colors
    /// (as circles) and the two orange tones (as narrow diamonds) in Oklab's
    /// chroma hue plane (which really is the same as the a/b plane, i.e., they
    /// both have the exact same colors in the exact same positions.
    ///
    /// ![The colors plotted on Oklab's chroma and hue plane](https://raw.githubusercontent.com/apparebit/prettypretty/main/docs/figures/vga-colors.svg)
    ///
    /// Given the very limited repertoire of ANSI colors, changes in lightness,
    /// chroma, and hue seem unavoidable when mapping arbitrary colors to ANSI
    /// colors. But white still seems like a worse choice than that dark orange.
    /// Let's explore if we can do better by prioritizing hue. We'll
    ///
    /// In fact, let's explore that idea a little further. We'll be comparing
    /// colors in Oklrch. So, we prepare a list with the color values for the 16
    /// extended ANSI colors in that color space. That, by the way, is pretty
    /// much what [`ColorMatcher::new`] does as well.
    /// ```
    /// # use prettypretty::{AnsiColor, Color, ColorFormatError, ColorSpace, DEFAULT_THEME};
    /// # use std::str::FromStr;
    /// let ansi_colors: Vec<Color> = (0..=15)
    ///     .map(|n| {
    ///         DEFAULT_THEME[AnsiColor::try_from(n).unwrap()]
    ///             .to(ColorSpace::Oklrch)
    ///     })
    ///     .collect();
    /// ```
    ///
    /// Next, we need a function that calculates the distance between the
    /// coordinates of two colors in Oklrch. Since we are doing exploratory
    /// coding, we exclusively focus on hue and calculate the minimum degree of
    /// separation. Degrees being circular, computing the remainder of the
    /// difference is not enough. We need to consider both differences.
    /// ```
    /// fn minimum_degrees_of_separation(c1: &[f64; 3], c2: &[f64; 3]) -> f64 {
    ///     (c1[2] - c2[2]).rem_euclid(360.0)
    ///         .min((c2[2] - c1[2]).rem_euclid(360.0))
    /// }
    /// ```
    ///
    /// That's it. We have everything we need. All that's left to do is to
    /// instantiate the same yellow again and find the closest matching color
    /// on our list with our distance metric.
    ///
    /// ```
    /// # use prettypretty::{AnsiColor, Color, ColorFormatError, ColorSpace, DEFAULT_THEME};
    /// # use std::str::FromStr;
    /// # let ansi_colors: Vec<Color> = (0..=15)
    /// #     .map(|n| {
    /// #         DEFAULT_THEME[AnsiColor::try_from(n).unwrap()]
    /// #             .to(ColorSpace::Oklrch)
    /// #     })
    /// #     .collect();
    /// # fn minimum_degrees_of_separation(c1: &[f64; 3], c2: &[f64; 3]) -> f64 {
    /// #     (c1[2] - c2[2]).rem_euclid(360.0)
    /// #         .min((c2[2] - c1[2]).rem_euclid(360.0))
    /// # }
    /// let yellow = Color::from_str("#ffa563")?;
    /// let closest = yellow.find_closest(
    ///     &ansi_colors,
    ///     ColorSpace::Oklrch,
    ///     minimum_degrees_of_separation,
    /// ).unwrap();
    /// assert_eq!(closest, 3);
    /// # Ok::<(), ColorFormatError>(())
    /// ```
    /// <div class=color-swatch>
    /// <div style="background-color: #ffa563;"></div>
    /// <div style="background-color: #a50;"></div>
    /// </div>
    /// <br>
    ///
    /// The hue-based comparison picks ANSI color 3, yellow, which looks great
    /// on the color swatch. A [quick
    /// check](https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit)
    /// of VGA text mode colors validates that the brown tone is the closest
    /// color indeed—even if it is much darker. That suggests that a hue-based
    /// distance metric is both feasible and desirable.
    pub fn to_ansi(&self, color: &Color) -> AnsiColor {
        use crate::color::core::{delta_e_ok, find_closest};

        let color = color.to(self.space);
        find_closest(color.coordinates(), &self.ansi, delta_e_ok)
            .map(|idx| AnsiColor::try_from(idx as u8).unwrap())
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
    /// # use prettypretty::{EmbeddedRgb, OutOfBoundsError, ColorMatcher, OkVersion};
    /// let matcher = ColorMatcher::new(&DEFAULT_THEME, OkVersion::Revised);
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
    ///             assert!((color[0] - c1).abs() < f64::EPSILON);
    ///
    ///             let result = matcher.to_eight_bit(&color);
    ///             assert_eq!(result, EightBitColor::Rgb(embedded));
    ///         }
    ///     }
    /// }
    /// # Ok::<(), OutOfBoundsError>(())
    /// ```
    pub fn to_eight_bit(&self, color: &Color) -> EightBitColor {
        use crate::color::core::{delta_e_ok, find_closest};

        let color = color.to(self.space);
        find_closest(color.coordinates(), &self.eight_bit, delta_e_ok)
            .map(|idx| EightBitColor::from(idx as u8 + 16))
            .unwrap()
    }
}

// ====================================================================================================================

#[cfg(test)]
mod test {
    use crate::{AnsiColor, Color, ColorMatcher, OkVersion, OutOfBoundsError, DEFAULT_THEME};

    #[test]
    fn test_matcher() -> Result<(), OutOfBoundsError> {
        let matcher = ColorMatcher::new(&DEFAULT_THEME, OkVersion::Revised);

        let result = matcher.to_ansi(&Color::srgb(1.0, 1.0, 0.0));
        assert_eq!(result, AnsiColor::BrightYellow);

        Ok(())
    }
}
use crate::Float;
use super::{normalize, ColorSpace};

/// Convert the given 24-bit RGB coordinates to floating point coordinates.
pub(crate) fn from_24bit(r: u8, g: u8, b: u8) -> [Float; 3] {
    [r as Float / 255.0, g as Float / 255.0, b as Float / 255.0]
}

/// Convert the color coordinates to 24-bit representation.
///
/// This function converts the color coordinates to 24-bit representation. It
/// assumes that the color is an in-gamut RGB color, i.e., that its coordinates
/// range `0..=1`. Even if that is not the case, the conversion automatically
/// clamps coordinates to the range `0x00..=0xff`.
pub(crate) fn to_24bit(space: ColorSpace, coordinates: &[Float; 3]) -> [u8; 3] {
    let [r, g, b] = normalize(space, coordinates);
    [
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    ]
}

// --------------------------------------------------------------------------------------------------------------------

/// Multiply the 3 by 3 matrix and 3-element vector with each other, producing a
/// new 3-element vector.
#[inline]
fn multiply(matrix: &[[Float; 3]; 3], vector: &[Float; 3]) -> [Float; 3] {
    let [row1, row2, row3] = matrix;

    [
        row1[0].mul_add(vector[0], row1[1].mul_add(vector[1], row1[2] * vector[2])),
        row2[0].mul_add(vector[0], row2[1].mul_add(vector[1], row2[2] * vector[2])),
        row3[0].mul_add(vector[0], row3[1].mul_add(vector[1], row3[2] * vector[2])),
    ]
}

// --------------------------------------------------------------------------------------------------------------------

/// Convert coordinates from gamma-corrected RGB to linear RGB using sRGB's
/// gamma. Display P3 uses the very same gamma. This is a one-hop, direct
/// conversion.
#[inline]
fn rgb_to_linear_rgb(value: &[Float; 3]) -> [Float; 3] {
    #[inline]
    fn convert(value: Float) -> Float {
        let magnitude = value.abs();
        if magnitude <= 0.04045 {
            value / 12.92
        } else {
            ((magnitude + 0.055) / 1.055).powf(2.4).copysign(value)
        }
    }

    [convert(value[0]), convert(value[1]), convert(value[2])]
}

/// Convert coordinates from linear RGB to gamma-corrected RGB using sRGB's
/// gamma. Display P3 uses the very same gamma. This is a one-hop, direct
/// conversion.
#[inline]
fn linear_rgb_to_rgb(value: &[Float; 3]) -> [Float; 3] {
    #[inline]
    fn convert(value: Float) -> Float {
        let magnitude = value.abs();
        if magnitude <= 0.00313098 {
            value * 12.92
        } else {
            (magnitude.powf(1.0 / 2.4) * 1.055 - 0.055).copysign(value)
        }
    }

    [convert(value[0]), convert(value[1]), convert(value[2])]
}

// --------------------------------------------------------------------------------------------------------------------
// https://github.com/color-js/color.js/blob/a77e080a070039c534dda3965a769675aac5f75e/src/spaces/srgb-linear.js

#[rustfmt::skip]
#[allow(clippy::excessive_precision)]
const LINEAR_SRGB_TO_XYZ: [[Float; 3]; 3] = [
    [ 0.41239079926595934, 0.357584339383878,   0.1804807884018343  ],
    [ 0.21263900587151027, 0.715168678767756,   0.07219231536073371 ],
    [ 0.01933081871559182, 0.11919477979462598, 0.9505321522496607  ],
];

/// Convert coordinates for linear sRGB to XYZ. This is a one-hop, direct conversion.
#[inline]
fn linear_srgb_to_xyz(value: &[Float; 3]) -> [Float; 3] {
    multiply(&LINEAR_SRGB_TO_XYZ, value)
}

// https://github.com/color-js/color.js/blob/a77e080a070039c534dda3965a769675aac5f75e/src/spaces/srgb-linear.js

#[rustfmt::skip]
#[allow(clippy::excessive_precision)]
const XYZ_TO_LINEAR_SRGB: [[Float; 3]; 3] = [
	[  3.2409699419045226,  -1.537383177570094,   -0.4986107602930034  ],
	[ -0.9692436362808796,   1.8759675015077202,   0.04155505740717559 ],
	[  0.05563007969699366, -0.20397695888897652,  1.0569715142428786  ],
];

/// Convert coordinates for XYZ to linear sRGB. THis is a one-hop, direct
/// conversion.
#[inline]
fn xyz_to_linear_srgb(value: &[Float; 3]) -> [Float; 3] {
    multiply(&XYZ_TO_LINEAR_SRGB, value)
}

// --------------------------------------------------------------------------------------------------------------------
// https://github.com/color-js/color.js/blob/a77e080a070039c534dda3965a769675aac5f75e/src/spaces/p3-linear.js

#[rustfmt::skip]
#[allow(clippy::excessive_precision)]
const LINEAR_DISPLAY_P3_TO_XYZ: [[Float; 3]; 3] = [
    [ 0.4865709486482162, 0.26566769316909306, 0.1982172852343625 ],
    [ 0.2289745640697488, 0.6917385218365064,  0.079286914093745  ],
    [ 0.0000000000000000, 0.04511338185890264, 1.043944368900976  ],
];

/// Convert coordinates for linear Display P3 to XYZ. This is a one-hop, direct
/// conversion.
#[inline]
fn linear_display_p3_to_xyz(value: &[Float; 3]) -> [Float; 3] {
    multiply(&LINEAR_DISPLAY_P3_TO_XYZ, value)
}

// https://github.com/color-js/color.js/blob/a77e080a070039c534dda3965a769675aac5f75e/src/spaces/p3-linear.js

#[rustfmt::skip]
#[allow(clippy::excessive_precision)]
const XYZ_TO_LINEAR_DISPLAY_P3: [[Float; 3]; 3] = [
    [  2.493496911941425,   -0.9313836179191239,  -0.40271078445071684  ],
    [ -0.8294889695615747,   1.7626640603183463,   0.023624685841943577 ],
    [  0.03584583024378447, -0.07617238926804182,  0.9568845240076872   ],
];

/// Convert coordinates for XYZ to linear Display P3. This is a one-hop, direct
/// conversion.
#[inline]
fn xyz_to_linear_display_p3(value: &[Float; 3]) -> [Float; 3] {
    multiply(&XYZ_TO_LINEAR_DISPLAY_P3, value)
}

// --------------------------------------------------------------------------------------------------------------------
// https://github.com/color-js/color.js/blob/main/src/spaces/rec2020.js

mod rec2020 {
    use crate::Float;

    #[allow(clippy::excessive_precision)]
    const ALPHA: Float = 1.09929682680944;
    #[allow(clippy::excessive_precision)]
    const BETA: Float = 0.018053968510807;

    /// Convert coordinates for Rec. 2020 to linear Rec. 2020. This is a
    /// one-hop, direct conversion.
    #[inline]
    pub(super) fn rec2020_to_linear_rec2020(value: &[Float; 3]) -> [Float; 3] {
        #[inline]
        fn convert(value: Float) -> Float {
            if value < BETA * 4.5 {
                value / 4.5
            } else {
                ((value + ALPHA - 1.0) / ALPHA).powf((0.45 as Float).recip())
            }
        }

        [convert(value[0]), convert(value[1]), convert(value[2])]
    }

    /// Convert coordinates for linear Rec. 2020 to Rec. 2020. This is a
    /// one-hop, direct conversion.
    #[inline]
    pub(super) fn linear_rec2020_to_rec2020(value: &[Float; 3]) -> [Float; 3] {
        #[inline]
        fn convert(value: Float) -> Float {
            if value < BETA {
                value * 4.5
            } else {
                ALPHA * value.powf(0.45) - (ALPHA - 1.0)
            }
        }

        [convert(value[0]), convert(value[1]), convert(value[2])]
    }
}

use rec2020::{linear_rec2020_to_rec2020, rec2020_to_linear_rec2020};

// --------------------------------------------------------------------------------------------------------------------
// https://github.com/color-js/color.js/blob/main/src/spaces/rec2020-linear.js

#[rustfmt::skip]
#[allow(clippy::excessive_precision)]
const LINEAR_REC2020_TO_XYZ: [[Float; 3]; 3] = [
	[ 0.6369580483012914, 0.14461690358620832,  0.1688809751641721  ],
	[ 0.2627002120112671, 0.6779980715188708,   0.05930171646986196 ],
	[ 0.000000000000000,  0.028072693049087428, 1.060985057710791   ],
];

/// Convert coordinates for linear Rec. 2020 to XYZ. This is a one-hop, direct
/// conversion.
#[inline]
fn linear_rec2020_to_xyz(value: &[Float; 3]) -> [Float; 3] {
    multiply(&LINEAR_REC2020_TO_XYZ, value)
}

#[rustfmt::skip]
#[allow(clippy::excessive_precision)]
const XYZ_TO_LINEAR_REC2020: [[Float; 3]; 3] = [
	[  1.716651187971268,  -0.355670783776392, -0.253366281373660  ],
	[ -0.666684351832489,   1.616481236634939,  0.0157685458139111 ],
	[  0.017639857445311,  -0.042770613257809,  0.942103121235474  ],
];

/// Convert coordinates for XYZ to linear Rec. 2020. This is a one-hop, direct
/// conversion.
#[inline]
fn xyz_to_linear_rec2020(value: &[Float; 3]) -> [Float; 3] {
    multiply(&XYZ_TO_LINEAR_REC2020, value)
}

// --------------------------------------------------------------------------------------------------------------------

mod oklab {
    use crate::Float;

    /// Convert coordinates for Oklch to Oklab or for Oklrch to Oklrab. This is a
    /// one-hop, direct conversion.
    #[inline]
    #[allow(non_snake_case)]
    pub(crate) fn okxch_to_okxab(value: &[Float; 3]) -> [Float; 3] {
        let [L, C, h] = *value;

        if h.is_nan() {
            [L, 0.0, 0.0]
        } else {
            let hue_radian = h.to_radians();
            [L, C * hue_radian.cos(), C * hue_radian.sin()]
        }
    }

    const EPSILON: Float = 0.0002;

    /// Convert coordinates for Oklab to Oklch or for Oklrab to Oklrch. This is a
    /// one-hop, direct conversion.
    #[inline]
    #[allow(non_snake_case)]
    pub(super) fn okxab_to_okxch(value: &[Float; 3]) -> [Float; 3] {
        let [L, a, b] = *value;
        let (C, h) = if a.abs() < EPSILON && b.abs() < EPSILON {
            (0.0, Float::NAN)
        } else {
            ((a.powi(2) + b.powi(2)).sqrt(), b.atan2(a).to_degrees())
        };

        let h = if h < 0.0 { h + 360.0 } else { h };

        [L, C, h]
    }

    const K1: Float = 0.206;
    const K2: Float = 0.03;
    const K3: Float = (1.0 + K1) / (1.0 + K2);

    /// Convert coordinates for Oklab to Oklrab or for Oklch to Oklrch. This
    /// function replaces the lightness L with the [improved lightness
    /// Lr](https://bottosson.github.io/posts/colorpicker/#intermission---a-new-lightness-estimate-for-oklab).
    /// This is a one-hop, direct conversion.
    #[inline]
    pub(super) fn oklxx_to_oklrxx(value: &[Float; 3]) -> [Float; 3] {
        let [l, a, b] = *value;
        let k3l = K3 * l;
        [
            0.5 * (k3l - K1 + ((k3l - K1) * (k3l - K1) + 4.0 * K2 * k3l).sqrt()),
            a,
            b,
        ]
    }

    /// Convert coordinates for Oklrab to Oklab or for Oklrch to Oklch. This
    /// function replaces the [improved lightness
    /// Lr](https://bottosson.github.io/posts/colorpicker/#intermission---a-new-lightness-estimate-for-oklab)
    /// with the original lightness L. This is a one-hop, direct conversion.
    #[inline]
    pub(super) fn oklrxx_to_oklxx(value: &[Float; 3]) -> [Float; 3] {
        let [lr, a, b] = *value;
        [(lr * (lr + K1)) / (K3 * (lr + K2)), a, b]
    }
}

pub(crate) use oklab::okxch_to_okxab;
use oklab::{oklrxx_to_oklxx, oklxx_to_oklrxx, okxab_to_okxch};

// --------------------------------------------------------------------------------------------------------------------
// https://github.com/color-js/color.js/blob/a77e080a070039c534dda3965a769675aac5f75e/src/spaces/oklab.js

#[rustfmt::skip]
#[allow(clippy::excessive_precision)]
const OKLAB_TO_OKLMS: [[Float; 3]; 3] = [
    [ 1.0000000000000000,  0.3963377773761749,  0.2158037573099136 ],
    [ 1.0000000000000000, -0.1055613458156586, -0.0638541728258133 ],
    [ 1.0000000000000000, -0.0894841775298119, -1.2914855480194092 ],
];

#[rustfmt::skip]
#[allow(clippy::excessive_precision)]
const OKLMS_TO_XYZ: [[Float; 3]; 3] = [
    [  1.2268798758459243, -0.5578149944602171,  0.2813910456659647 ],
    [ -0.0405757452148008,  1.1122868032803170, -0.0717110580655164 ],
    [ -0.0763729366746601, -0.4214933324022432,  1.5869240198367816 ],
];

/// Convert coordinates for Oklab to XYZ. This is a one-hop, direct conversion,
/// even though it requires two matrix multiplications and a coordinate-wise
/// exponential.
#[inline]
fn oklab_to_xyz(value: &[Float; 3]) -> [Float; 3] {
    let [l, m, s] = multiply(&OKLAB_TO_OKLMS, value);
    multiply(&OKLMS_TO_XYZ, &[l.powi(3), m.powi(3), s.powi(3)])
}

// https://github.com/color-js/color.js/blob/a77e080a070039c534dda3965a769675aac5f75e/src/spaces/oklab.js

#[rustfmt::skip]
#[allow(clippy::excessive_precision)]
const XYZ_TO_OKLMS: [[Float; 3]; 3] = [
    [ 0.8190224379967030, 0.3619062600528904, -0.1288737815209879 ],
    [ 0.0329836539323885, 0.9292868615863434,  0.0361446663506424 ],
    [ 0.0481771893596242, 0.2642395317527308,  0.6335478284694309 ],
];

#[rustfmt::skip]
#[allow(clippy::excessive_precision)]
const OKLMS_TO_OKLAB: [[Float; 3]; 3] = [
    [ 0.2104542683093140,  0.7936177747023054, -0.0040720430116193 ],
    [ 1.9779985324311684, -2.4285922420485799,  0.4505937096174110 ],
    [ 0.0259040424655478,  0.7827717124575296, -0.8086757549230774 ],
];

/// Convert coordinates for XYZ to Oklab. This is a one-hop, direct conversion,
/// even though it requires two matrix multiplications and a coordinate-wise
/// exponential.
#[inline]
fn xyz_to_oklab(value: &[Float; 3]) -> [Float; 3] {
    let [l, m, s] = multiply(&XYZ_TO_OKLMS, value);
    multiply(&OKLMS_TO_OKLAB, &[l.cbrt(), m.cbrt(), s.cbrt()])
}

// --------------------------------------------------------------------------------------------------------------------

/// Convert coordinates for sRGB to XYZ. This is a two-hop conversion.
#[inline]
fn srgb_to_xyz(value: &[Float; 3]) -> [Float; 3] {
    let linear_srgb = rgb_to_linear_rgb(value);
    linear_srgb_to_xyz(&linear_srgb)
}

/// Convert coordinates for XYZ to sRGB. This is a two-hop conversion.
#[inline]
fn xyz_to_srgb(value: &[Float; 3]) -> [Float; 3] {
    let linear_srgb = xyz_to_linear_srgb(value);
    linear_rgb_to_rgb(&linear_srgb)
}

/// Convert coordinates for Display P3 to XYZ. This is a two-hop conversion.
#[inline]
fn display_p3_to_xyz(value: &[Float; 3]) -> [Float; 3] {
    let linear_p3 = rgb_to_linear_rgb(value);
    linear_display_p3_to_xyz(&linear_p3)
}

/// Convert coordinates for XYZ to Display P3. This is a two-hop conversion.
#[inline]
fn xyz_to_display_p3(value: &[Float; 3]) -> [Float; 3] {
    let linear_p3 = xyz_to_linear_display_p3(value);
    linear_rgb_to_rgb(&linear_p3)
}

/// Convert coordinates for Rec. 2020 to XYZ. This is a two-hop conversion.
#[inline]
fn rec2020_to_xyz(value: &[Float; 3]) -> [Float; 3] {
    let linear_rec2020 = rec2020_to_linear_rec2020(value);
    linear_rec2020_to_xyz(&linear_rec2020)
}

/// Convert coordinates for XYZ to Rec. 2020. This is a two-hop conversion.
#[inline]
fn xyz_to_rec2020(value: &[Float; 3]) -> [Float; 3] {
    let linear_rec2020 = xyz_to_linear_rec2020(value);
    linear_rec2020_to_rec2020(&linear_rec2020)
}

/// Convert coordinates for Oklch to XYZ. This is a two-hop conversion.
#[inline]
fn oklch_to_xyz(value: &[Float; 3]) -> [Float; 3] {
    let oklab = okxch_to_okxab(value);
    oklab_to_xyz(&oklab)
}

/// Convert coordinates for XYZ to Oklch. This is a two-hop conversion.
#[inline]
fn xyz_to_oklch(value: &[Float; 3]) -> [Float; 3] {
    let oklab = xyz_to_oklab(value);
    okxab_to_okxch(&oklab)
}

/// Convert coordinates for Oklrab to XYZ. This is a two-hop conversion.
#[inline]
fn oklrab_to_xyz(value: &[Float; 3]) -> [Float; 3] {
    let oklab = oklrxx_to_oklxx(value);
    oklab_to_xyz(&oklab)
}
/// Convert coordinates for XYZ to Oklrab. This is a two-hop conversion.
#[inline]
fn xyz_to_oklrab(value: &[Float; 3]) -> [Float; 3] {
    let oklab = xyz_to_oklab(value);
    oklxx_to_oklrxx(&oklab)
}

/// Convert coordinates for Oklab to Oklrch. This is a two-hop conversion.
#[inline]
fn oklab_to_oklrch(value: &[Float; 3]) -> [Float; 3] {
    let oklch = okxab_to_okxch(value);
    oklxx_to_oklrxx(&oklch)
}

/// Convert coordinates for Oklrch to Oklab. This is a two-hop conversion.
#[inline]
fn oklrch_to_oklab(value: &[Float; 3]) -> [Float; 3] {
    let oklch = oklrxx_to_oklxx(value);
    okxch_to_okxab(&oklch)
}

/// Convert coordinates for Oklrab to Oklch. This is a two-hop conversion.
#[inline]
fn oklrab_to_oklch(value: &[Float; 3]) -> [Float; 3] {
    let oklab = oklrxx_to_oklxx(value);
    okxab_to_okxch(&oklab)
}

/// Convert coordinates for Oklch to Oklrab. This is a two-hop conversion.
#[inline]
fn oklch_to_oklrab(value: &[Float; 3]) -> [Float; 3] {
    let oklab = okxch_to_okxab(value);
    oklxx_to_oklrxx(&oklab)
}

// --------------------------------------------------------------------------------------------------------------------

/// Convert coordinates for Oklrch to XYZ. This is a three-hop conversion.
#[inline]
fn oklrch_to_xyz(value: &[Float; 3]) -> [Float; 3] {
    let oklch = oklrxx_to_oklxx(value);
    oklch_to_xyz(&oklch)
}
/// Convert coordinates for XYZ to Oklrab. This is a three-hop conversion.
#[inline]
fn xyz_to_oklrch(value: &[Float; 3]) -> [Float; 3] {
    let oklch = xyz_to_oklch(value);
    oklxx_to_oklrxx(&oklch)
}

// --------------------------------------------------------------------------------------------------------------------

/// Convert the coordinates from one color space to another.
///
/// This function normalizes not-a-number coordinates to zero and then converts
/// them to to the targeted color space, which may be the same as the original
/// color space. This function does not check whether the result is in gamut for
/// the targeted color space.
#[must_use = "function returns new color coordinates and does not mutate original value"]
pub(crate) fn convert(
    from_space: ColorSpace,
    to_space: ColorSpace,
    coordinates: &[Float; 3],
) -> [Float; 3] {
    use ColorSpace::*;

    // 1. Normalize coordinates. Be done if color spaces are the same.
    let coordinates = normalize(from_space, coordinates);
    if from_space == to_space {
        return coordinates;
    }

    // 2. Handle in-branch conversions that don't go through root XYZ
    match (from_space, to_space) {
        // Single-hop sRGB and P3 conversions
        (Srgb, LinearSrgb) | (DisplayP3, LinearDisplayP3) => {
            return rgb_to_linear_rgb(&coordinates);
        }
        (LinearSrgb, Srgb) | (LinearDisplayP3, DisplayP3) => {
            return linear_rgb_to_rgb(&coordinates);
        }

        // Single-hop Rec2020 conversions
        (Rec2020, LinearRec2020) => return rec2020_to_linear_rec2020(&coordinates),
        (LinearRec2020, Rec2020) => return linear_rec2020_to_rec2020(&coordinates),

        // Single-hop Oklab variation conversions
        (Oklch, Oklab) | (Oklrch, Oklrab) => return okxch_to_okxab(&coordinates),
        (Oklab, Oklch) | (Oklrab, Oklrch) => return okxab_to_okxch(&coordinates),
        (Oklab, Oklrab) | (Oklch, Oklrch) => return oklxx_to_oklrxx(&coordinates),
        (Oklrab, Oklab) | (Oklrch, Oklch) => return oklrxx_to_oklxx(&coordinates),

        // Two-hop Oklab variation conversions
        (Oklrch, Oklab) => return oklrch_to_oklab(&coordinates),
        (Oklch, Oklrab) => return oklch_to_oklrab(&coordinates),
        (Oklab, Oklrch) => return oklab_to_oklrch(&coordinates),
        (Oklrab, Oklch) => return oklrab_to_oklch(&coordinates),
        _ => (),
    };

    // 3a. Convert from source color space to root XYZ
    let intermediate = match from_space {
        Srgb => srgb_to_xyz(&coordinates),
        LinearSrgb => linear_srgb_to_xyz(&coordinates),
        DisplayP3 => display_p3_to_xyz(&coordinates),
        LinearDisplayP3 => linear_display_p3_to_xyz(&coordinates),
        Rec2020 => rec2020_to_xyz(&coordinates),
        LinearRec2020 => linear_rec2020_to_xyz(&coordinates),
        Oklch => oklch_to_xyz(&coordinates),
        Oklab => oklab_to_xyz(&coordinates),
        Oklrch => oklrch_to_xyz(&coordinates),
        Oklrab => oklrab_to_xyz(&coordinates),
        Xyz => coordinates,
    };

    // 3b. Convert from root XYZ to target color space on different branch
    match to_space {
        Srgb => xyz_to_srgb(&intermediate),
        LinearSrgb => xyz_to_linear_srgb(&intermediate),
        DisplayP3 => xyz_to_display_p3(&intermediate),
        LinearDisplayP3 => xyz_to_linear_display_p3(&intermediate),
        Rec2020 => xyz_to_rec2020(&intermediate),
        LinearRec2020 => xyz_to_linear_rec2020(&intermediate),
        Oklch => xyz_to_oklch(&intermediate),
        Oklab => xyz_to_oklab(&intermediate),
        Oklrch => xyz_to_oklrch(&intermediate),
        Oklrab => xyz_to_oklrab(&intermediate),
        Xyz => intermediate,
    }
}

#[cfg(test)]
#[allow(clippy::excessive_precision)]
mod test {
    use super::*;
    use crate::core::test_util::close_enough;
    use crate::Float;

    struct Representations {
        srgb: [Float; 3],
        linear_srgb: [Float; 3],
        p3: [Float; 3],
        linear_p3: [Float; 3],
        rec2020: [Float; 3],
        linear_rec2020: [Float; 3],
        oklch: [Float; 3],
        oklab: [Float; 3],
        oklrch: [Float; 3],
        oklrab: [Float; 3],
        xyz: [Float; 3],
    }

    const BLACK: Representations = Representations {
        // #000000
        srgb: [0.0, 0.0, 0.0],
        linear_srgb: [0.0, 0.0, 0.0],
        p3: [0.0, 0.0, 0.0],
        linear_p3: [0.0, 0.0, 0.0],
        rec2020: [0.0, 0.0, 0.0],
        linear_rec2020: [0.0, 0.0, 0.0],
        oklch: [0.0, 0.0, Float::NAN],
        oklab: [0.0, 0.0, 0.0],
        oklrch: [0.0, 0.0, Float::NAN],
        oklrab: [0.0, 0.0, 0.0],
        xyz: [0.0, 0.0, 0.0],
    };

    const YELLOW: Representations = Representations {
        // #ffca00
        srgb: [1.0, 0.792156862745098, 0.0],
        linear_srgb: [1.0, 0.5906188409193369, 0.0],
        p3: [0.967346220711791, 0.8002244967941964, 0.27134084647161244],
        linear_p3: [0.9273192749713864, 0.6042079205196976, 0.059841923211596565],
        rec2020: [0.9071245864481046, 0.7821891940186851, 0.22941491945066222],
        linear_rec2020: [0.8218846623958427, 0.6121951716762088, 0.0683737567590739],
        oklch: [0.8613332073307732, 0.1760097742886813, 89.440876452466],
        oklab: [
            0.8613332073307732,
            0.0017175723640959761,
            0.17600139371700052,
        ],
        oklrch: [0.8385912822460642, 0.1760097742886813, 89.440876452466],
        oklrab: [
            0.8385912822460642,
            0.0017175723640959761,
            0.17600139371700052,
        ],
        xyz: [0.6235868473237722, 0.635031101987136, 0.08972950140152941],
    };

    const BLUE: Representations = Representations {
        // #3178ea
        srgb: [0.19215686274509805, 0.47058823529411764, 0.9176470588235294],
        linear_srgb: [
            0.030713443732993635,
            0.18782077230067787,
            0.8227857543962835,
        ],
        p3: [0.26851535563550943, 0.4644576150842869, 0.8876966971452301],
        linear_p3: [0.058605969547446124, 0.18260572039525869, 0.763285235993837],
        rec2020: [0.318905170074285, 0.4141244051667745, 0.8687817570254107],
        linear_rec2020: [0.11675330225613656, 0.18417975425846383, 0.7539171810709095],
        oklch: [0.5909012953108558, 0.18665606306724153, 259.66681920272595],
        oklab: [
            0.5909012953108558,
            -0.03348086515869664,
            -0.1836287492414715,
        ],
        oklrch: [0.5253778775789848, 0.18665606306724153, 259.66681920272595],
        oklrab: [
            0.5253778775789848,
            -0.03348086515869664,
            -0.1836287492414715,
        ],
        xyz: [0.22832473003420622, 0.20025321836938534, 0.80506528557483],
    };

    const WHITE: Representations = Representations {
        // #ffffff
        srgb: [1.0, 1.0, 1.0],
        linear_srgb: [1.0, 1.0, 1.0],
        p3: [0.9999999999999999, 0.9999999999999997, 0.9999999999999999],
        linear_p3: [1.0, 0.9999999999999998, 1.0],
        rec2020: [1.0000000000000002, 1.0, 1.0],
        linear_rec2020: [1.0000000000000004, 1.0, 0.9999999999999999],
        oklch: [1.0000000000000002, 0.0, Float::NAN],
        oklab: [1.0000000000000002, -4.996003610813204e-16, 0.0],
        xyz: [0.9504559270516717, 1.0, 1.0890577507598784],
        oklrch: [1.0000000000000002, 0.0, Float::NAN],
        oklrab: [1.0000000000000002, 0.0, 0.0],
    };

    #[test]
    fn test_conversions() {
        for &color in [&BLACK, &YELLOW, &BLUE, &WHITE].iter() {
            // Test all one-hop conversions
            let linear_srgb = rgb_to_linear_rgb(&color.srgb);
            assert!(close_enough(&linear_srgb, &color.linear_srgb, false));

            let srgb = linear_rgb_to_rgb(&linear_srgb);
            assert!(close_enough(&srgb, &color.srgb, false));

            let xyz = linear_srgb_to_xyz(&linear_srgb);
            assert!(close_enough(&xyz, &color.xyz, false));

            let also_linear_srgb = xyz_to_linear_srgb(&xyz);
            assert!(close_enough(&also_linear_srgb, &linear_srgb, false));

            let linear_p3 = xyz_to_linear_display_p3(&xyz);
            assert!(close_enough(&linear_p3, &color.linear_p3, false));

            let also_xyz = linear_display_p3_to_xyz(&linear_p3);
            assert!(close_enough(&also_xyz, &xyz, false));

            let p3 = linear_rgb_to_rgb(&linear_p3);
            assert!(close_enough(&p3, &color.p3, false));

            let also_linear_p3 = rgb_to_linear_rgb(&p3);
            assert!(close_enough(&also_linear_p3, &linear_p3, false));

            let linear_rec2020 = xyz_to_linear_rec2020(&xyz);
            assert!(close_enough(&linear_rec2020, &color.linear_rec2020, false));

            let and_also_xyz = linear_rec2020_to_xyz(&linear_rec2020);
            assert!(close_enough(&and_also_xyz, &xyz, false));

            let rec2020 = linear_rec2020_to_rec2020(&linear_rec2020);
            assert!(close_enough(&rec2020, &color.rec2020, false));

            let also_linear_rec2020 = rec2020_to_linear_rec2020(&rec2020);
            assert!(close_enough(&also_linear_rec2020, &linear_rec2020, false));

            let oklab = xyz_to_oklab(&xyz);
            assert!(close_enough(&oklab, &color.oklab, false));

            let and_again_xyz = oklab_to_xyz(&oklab);
            assert!(close_enough(&and_again_xyz, &xyz, false));

            let oklch = okxab_to_okxch(&oklab);
            assert!(close_enough(dbg!(&oklch), dbg!(&color.oklch), true));

            let also_oklab = okxch_to_okxab(&oklch);
            assert!(close_enough(&also_oklab, &oklab, false));

            let oklrab = oklxx_to_oklrxx(&oklab);
            assert!(close_enough(&oklrab, &color.oklrab, false));

            let oklab_too = oklrxx_to_oklxx(&oklrab);
            assert!(close_enough(&oklab_too, &color.oklab, false));

            let oklrch = oklxx_to_oklrxx(&oklch);
            assert!(close_enough(&oklrch, &color.oklrch, true));

            let oklch_too = oklrxx_to_oklxx(&oklrch);
            assert!(close_enough(&oklch_too, &color.oklch, true));
        }
    }
}

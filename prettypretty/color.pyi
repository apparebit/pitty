from typing import Self

class AnsiColor:
    Black: AnsiColor = ...
    Red: AnsiColor = ...
    Green: AnsiColor = ...
    Yellow: AnsiColor = ...
    Blue: AnsiColor = ...
    Magenta: AnsiColor = ...
    Cyan: AnsiColor = ...
    White: AnsiColor = ...
    BrightBlack: AnsiColor = ...
    BrightRed: AnsiColor = ...
    BrightGreen: AnsiColor = ...
    BrightYellow: AnsiColor = ...
    BrightBlue: AnsiColor = ...
    BrightMagenta: AnsiColor = ...
    BrightCyan: AnsiColor = ...
    BrightWhite: AnsiColor = ...

    @staticmethod
    def from_8bit(value: int) -> AnsiColor:
        ...
    def to_8bit(self) -> int:
        ...
    def is_bright(self) -> bool:
        ...
    def nonbright(self) -> AnsiColor:
        ...
    def __hash__(self) -> int:
        ...
    def __eq__(self, other: object) -> bool:
        ...
    def __ne__(self, other: object) -> bool:
        ...


class EmbeddedRgb:
    def __new__(cls, r: int, g: int, b:int) -> Self:
        ...
    @staticmethod
    def from_8bit(value: int) -> EmbeddedRgb:
        ...
    def to_8bit(self) -> int:
        ...
    def to_color(self) -> Color:
        ...
    def coordinates(self) -> list[int]:
        ...
    def __len__(self) -> int:
        ...
    def __getitem__(self, index: int) -> int:
        ...
    def __hash__(self) -> int:
        ...
    def __eq__(self, other: object) -> bool:
        ...
    def __ne__(self, other: object) -> bool:
        ...
    def __repr__(self) -> str:
        ...


class GrayGradient:
    def __new__(cls, value: int) -> Self:
        ...
    @staticmethod
    def from_8bit(value: int) -> GrayGradient:
        ...
    def to_8bit(self) -> int:
        ...
    def to_color(self) -> Color:
        ...
    def level(self) -> int:
        ...
    def __hash__(self) -> int:
        ...
    def __eq__(self, other: object) -> bool:
        ...
    def __ne__(self, other: object) -> bool:
        ...
    def __gt__(self, other: object) -> bool:
        ...
    def __ge__(self, other: object) -> bool:
        ...
    def __lt__(self, other: object) -> bool:
        ...
    def __le__(self, other: object) -> bool:
        ...
    def __repr__(self) -> str:
        ...


class TrueColor:
    def __new__(cls, r: int, g: int, b: int) -> Self:
        ...
    @staticmethod
    def from_color(color: Color) -> TrueColor:
        ...
    def to_color(self) -> Color:
        ...
    def coordinates(self) -> list[int]:
        ...
    def __len__(self) -> int:
        ...
    def __getitem__(self, index: int) -> int:
        ...
    def __hash__(self) -> int:
        ...
    def __eq__(self, other: object) -> bool:
        ...
    def __ne__(self, other: object) -> bool:
        ...
    def __repr__(self) -> str:
        ...
    def __str__(self) -> str:
        ...


class TerminalColor_Default(TerminalColor):
    def __new__(cls) -> Self:
        ...


class TerminalColor_Ansi(TerminalColor):
    def __new__(cls, color:AnsiColor) -> Self:
        ...
    @property
    def color(self) -> AnsiColor:
        ...


class TerminalColor_Rgb6(TerminalColor):
    def __new__(cls, color: EmbeddedRgb) -> Self:
        ...
    @property
    def color(self) -> EmbeddedRgb:
        ...


class TerminalColor_Gray(TerminalColor):
    def __new__(cls, color: GrayGradient) -> Self:
        ...
    @property
    def color(self) -> GrayGradient:
        ...


class TerminalColor_Rgb256(TerminalColor):
    def __new__(cls, color: TrueColor) -> Self:
        ...
    @property
    def color(self) -> TrueColor:
        ...


class TerminalColor:
    Default = TerminalColor_Default
    Ansi = TerminalColor_Ansi
    Rgb6 = TerminalColor_Rgb6
    Gray = TerminalColor_Gray
    Rgb256 = TerminalColor_Rgb256

    @staticmethod
    def from_8bit(color: int) -> TerminalColor:
        ...
    @staticmethod
    def from_24bit(r: int, g: int, b: int) -> TerminalColor:
        ...
    @staticmethod
    def from_color(color: Color) -> TerminalColor:
        ...
    def to_8bit(self) -> int:
        ...
    def is_default(self) -> bool:
        ...
    def sgr_parameters(self, layer: Layer) -> list[int]:
        ...
    def __hash__(self) -> int:
        ...
    def __eq__(self, other: object) -> bool:
        ...
    def __ne__(self, other: object) -> bool:
        ...
    def __repr__(self) -> str:
        ...


class Fidelity:
    Plain: Fidelity = ...
    NoColor: Fidelity = ...
    Ansi: Fidelity = ...
    EightBit: Fidelity = ...
    Full: Fidelity = ...

    @staticmethod
    def from_color(color: TerminalColor) -> Fidelity:
        ...
    def __hash__(self) -> int:
        ...
    def __eq__(self, other: object) -> bool:
        ...
    def __ne__(self, other: object) -> bool:
        ...
    def __gt__(self, other: object) -> bool:
        ...
    def __ge__(self, other: object) -> bool:
        ...
    def __lt__(self, other: object) -> bool:
        ...
    def __le__(self, other: object) -> bool:
        ...
    def __str__(self) -> str:
        ...


class Layer:
    Foreground: Layer = ...
    Background: Layer = ...

    def offset(self) -> int:
        ...
    def __hash__(self) -> int:
        ...
    def __eq__(self, other: object) -> bool:
        ...
    def __ne__(self, other: object) -> bool:
        ...
    def __str__(self) -> str:
        ...


class ColorSpace:
    Srgb: ColorSpace = ...
    LinearSrgb: ColorSpace = ...
    DisplayP3: ColorSpace = ...
    LinearDisplayP3: ColorSpace = ...
    Rec2020: ColorSpace = ...
    LinearRec2020: ColorSpace = ...
    Oklab: ColorSpace = ...
    Oklch: ColorSpace = ...
    Oklrab: ColorSpace = ...
    Oklrch: ColorSpace = ...
    Xyz: ColorSpace = ...

    def is_rgb(self) -> bool:
        ...
    def is_polar(self) -> bool:
        ...
    def is_ok(self) -> bool:
        ...
    def is_bounded(self) -> bool:
        ...
    def __hash__(self) -> int:
        ...
    def __eq__(self, other: object) -> bool:
        ...
    def __ne__(self, other: object) -> bool:
        ...
    def __str__(self) -> str:
        ...


class HueInterpolation:
    Shorter: HueInterpolation = ...
    Longer: HueInterpolation = ...
    Increasing: HueInterpolation = ...
    Decreasing: HueInterpolation = ...

    def __hash__(self) -> int:
        ...
    def __eq__(self, other: object) -> bool:
        ...
    def __ne__(self, other: object) -> bool:
        ...


class OkVersion:
    Original: OkVersion = ...
    Revised: OkVersion = ...

    def cartesian_space(self) -> ColorSpace:
        ...
    def polar_space(self) -> ColorSpace:
        ...
    def __hash__(self) -> int:
        ...
    def __eq__(self, other: object) -> bool:
        ...
    def __ne__(self, other: object) -> bool:
        ...


class Color:
    def __new__(
        cls, space: ColorSpace, coordinates: tuple[float, float, float]
    ) -> Self:
        ...
    @staticmethod
    def parse(s: str) -> Color:
        ...
    @staticmethod
    def srgb(r: float, g: float, b: float) -> Color:
        ...
    @staticmethod
    def p3(r: float, g: float, b: float) -> Color:
        ...
    @staticmethod
    def oklab(l: float, a: float, b: float) -> Color:
        ...
    @staticmethod
    def oklrab(lr: float, a: float, b: float) -> Color:
        ...
    @staticmethod
    def oklch(l: float, c: float, h: float) -> Color:
        ...
    @staticmethod
    def oklrch(lr: float, c: float, h: float) -> Color:
        ...
    @staticmethod
    def from_24bit(r: int, g: int, b: int) -> Color:
        ...
    def to_24bit(self) -> list[int]:
        ...
    def to_hex_format(self) -> str:
        ...
    def is_default(self) -> bool:
        ...
    def space(self) -> ColorSpace:
        ...
    def coordinates(self) -> list[float]:
        ...
    def normalize(self) -> Self:
        ...
    def to(self, target: ColorSpace) -> Self:
        ...
    def in_gamut(self) -> bool:
        ...
    def clip(self) -> Self:
        ...
    def to_gamut(self) -> Self:
        ...
    def distance(self, other: Self, version: OkVersion) -> float:
        ...
    def interpolate(
        self,
        color: Self,
        interpolation_space: ColorSpace,
        interpolation_strategy: HueInterpolation,
    ) -> Interpolator:
        ...
    def lighten(self, factor: float) -> Self:
        ...
    def darken(self, factor: float) -> Self:
        ...
    def contrast_against(self, background: Self) -> float:
        ...
    def use_black_text(self) -> bool:
        ...
    def use_black_background(self) -> bool:
        ...
    def __hash__(self) -> int:
        ...
    def __eq__(self, other: object) -> bool:
        ...
    def __len__(self) -> int:
        ...
    def __getitem__(self, index: int) -> float:
        ...
    def __repr__(self) -> str:
        ...
    def __str__(self) -> str:
        ...


class Interpolator:
    def __new__(
        cls,
        color1: Color,
        color2: Color,
        space: ColorSpace,
        strategy: HueInterpolation
    ) -> Self:
        ...
    def at(self, fraction: float) -> Color:
        ...


class ThemeEntry:
    Foreground: ThemeEntry = ...
    Background: ThemeEntry = ...
    Black: ThemeEntry = ...
    Red: ThemeEntry = ...
    Green: ThemeEntry = ...
    Yellow: ThemeEntry = ...
    Blue: ThemeEntry = ...
    Magenta: ThemeEntry = ...
    Cyan: ThemeEntry = ...
    White: ThemeEntry = ...
    BrightBlack: ThemeEntry = ...
    BrightRed: ThemeEntry = ...
    BrightGreen: ThemeEntry = ...
    BrightYellow: ThemeEntry = ...
    BrightBlue: ThemeEntry = ...
    BrightMagenta: ThemeEntry = ...
    BrightCyan: ThemeEntry = ...
    BrightWhite: ThemeEntry = ...

    @staticmethod
    def from_index(index: int) -> ThemeEntry:
        ...
    @staticmethod
    def from_ansi_color(color: AnsiColor) -> ThemeEntry:
        ...
    def name(self) -> str:
        ...
    def __hash__(self) -> int:
        ...
    def __eq__(self, other: object) -> bool:
        ...
    def __ne__(self, other: object) -> bool:
        ...


class ThemeEntryIterator:
    def __iter__(self) -> Self:
        ...
    def __next__(self) -> None | ThemeEntry:
        ...


class Theme:
    @staticmethod
    def entries() -> ThemeEntryIterator:
        ...
    def __new__(cls, colors: list[Color]) -> Self:
        ...
    def __len__(self) -> int:
        ...
    def __getitem__(self, index: int) -> Color:
        ...
    def __repr__(self) -> str:
        ...


class Sampler:
    def __new__(cls, theme: Theme, ok_version: OkVersion) -> Self:
        ...
    def to_high_res_8bit(self, color: int) -> Color:
        ...
    def try_high_res(self, color: TerminalColor) -> None | Color:
        ...
    def to_high_res(self, color: TerminalColor, layer: Layer) -> Color:
        ...
    def to_closest_ansi(self, color: Color) -> AnsiColor:
        ...
    def to_ansi_in_rgb(self, color: Color) -> AnsiColor:
        ...
    def to_closest_8bit_raw(self, color: Color) -> int:
        ...
    def to_closest_8bit(self, color: Color) -> TerminalColor:
        ...
    def adjust(self, color: TerminalColor, fidelity: Fidelity) -> None | TerminalColor:
        ...

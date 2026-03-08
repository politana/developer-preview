use crate::{api::attr_style::AttributeValue, utils::math::FloatMod};

impl AttributeValue for f64 {
    fn attr_string(self) -> String {
        format!("{}", self)
    }
}

#[derive(Clone, Copy)]
pub enum AlignItems {
    Normal, Stretch, Center, Start, End, FlexStart, FlexEnd, Baseline
}

impl AttributeValue for AlignItems {
    fn attr_string(self) -> String {
        match self {
            AlignItems::Normal => "normal",
            AlignItems::Stretch => "stretch",
            AlignItems::Center => "center",
            AlignItems::Start => "start",
            AlignItems::End => "end",
            AlignItems::FlexStart => "flex-start",
            AlignItems::FlexEnd => "flex-end",
            AlignItems::Baseline => "baseline"
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub struct Angle {
    value: f64,
    unit: AngleUnit
}

#[derive(Clone, Copy)]
pub enum AngleUnit {
    Deg, Grad, Rad, Turn
}

impl Angle {
    pub fn new(value: f64, unit: AngleUnit) -> Self {
        Self { value, unit }
    }
}

pub trait IntoAngle {
    fn deg(self) -> Angle;
    fn grad(self) -> Angle;
    fn rad(self) -> Angle;
    fn turn(self) -> Angle;
}

impl <T> IntoAngle for T where T: Into<f64> {
    fn deg(self) -> Angle { Angle::new(self.into(), AngleUnit::Deg) }
    fn grad(self) -> Angle { Angle::new(self.into(), AngleUnit::Grad) }
    fn rad(self) -> Angle { Angle::new(self.into(), AngleUnit::Rad) }
    fn turn(self) -> Angle { Angle::new(self.into(), AngleUnit::Turn) }
}

impl AttributeValue for Angle {
    fn attr_string(self) -> String {
        format!(
            "{}{}",
            self.value,
            match self.unit {
                AngleUnit::Deg => "deg",
                AngleUnit::Grad => "grad",
                AngleUnit::Rad => "rad",
                AngleUnit::Turn => "turn"
            }
        )
    }
}

#[derive(Clone, Copy)]
pub enum BorderStyle {
    None, Hidden, Dotted, Dashed, Solid, Double, Groove, Ridge, Inset, Outset
}

impl AttributeValue for BorderStyle {
    fn attr_string(self) -> String {
        match self {
            BorderStyle::None => "none",
            BorderStyle::Hidden => "hidden",
            BorderStyle::Dotted => "dotted",
            BorderStyle::Dashed => "dashed",
            BorderStyle::Solid => "solid",
            BorderStyle::Double => "double",
            BorderStyle::Groove => "groove",
            BorderStyle::Ridge => "ridge",
            BorderStyle::Inset => "inset",
            BorderStyle::Outset => "outset"
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum BoxSizing { ContentBox, BorderBox }

impl AttributeValue for BoxSizing {
    fn attr_string(self) -> String {
        match self {
            BoxSizing::ContentBox => "content-box",
            BoxSizing::BorderBox => "border-box"
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum Color {
    Rgba(f64, f64, f64, f64),
    Hsla(f64, f64, f64, f64),
    Black, White, Transparent
}

impl AttributeValue for Color {
    fn attr_string(self) -> String {
        match self {
            Color::Rgba(r, g, b, a) => format!(
                "rgba({}, {}, {}, {})",
                r.clamp(0.0, 255.0),
                g.clamp(0.0, 255.0),
                b.clamp(0.0, 255.0),
                a.clamp(0.0, 1.0)
            ),
            Color::Hsla(h, s, l, a) => format!(
                "hsla({}, {}%, {}%, {})",
                h.fmod(360.0),
                s.clamp(0.0, 1.0) * 100.0,
                l.clamp(0.0, 1.0) * 100.0,
                a.clamp(0.0, 1.0)
            ),
            Color::Black => "black".to_string(),
            Color::White => "white".to_string(),
            Color::Transparent => "transparent".to_string()
        }
    }
}

#[derive(Clone, Copy)]
pub enum Cursor {
    Auto, Default, None, Pointer, Crosshair, Text,
    VerticalText, Grab, Grabbing, ColResize, RowResize
}

impl AttributeValue for Cursor {
    fn attr_string(self) -> String {
        match self {
            Cursor::Auto => "auto",
            Cursor::Default => "default",
            Cursor::None => "none",
            Cursor::Pointer => "pointer",
            Cursor::Crosshair => "crosshair",
            Cursor::Text => "text",
            Cursor::VerticalText => "vertical-text",
            Cursor::Grab => "grab",
            Cursor::Grabbing => "grabbing",
            Cursor::ColResize => "col-resize",
            Cursor::RowResize => "row-resize",
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum Display {
    Flex, InlineFlex, Grid, InlineGrid, Block, Inline, InlineBlock, Contents, None
}

impl AttributeValue for Display {
    fn attr_string(self) -> String {
        match self {
            Display::Flex => "flex",
            Display::InlineFlex => "inline-flex",
            Display::Grid => "grid",
            Display::InlineGrid => "inline-grid",
            Display::Block => "block",
            Display::Inline => "inline",
            Display::InlineBlock => "inline-block",
            Display::Contents => "contents",
            Display::None => "none"
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum FlexDirection { Row, RowReverse, Column, ColumnReverse }

impl AttributeValue for FlexDirection {
    fn attr_string(self) -> String {
        match self {
            FlexDirection::Row => "row",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::Column => "column",
            FlexDirection::ColumnReverse => "column-reverse",
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum FontOpticalSizing { Auto, None }

impl AttributeValue for FontOpticalSizing {
    fn attr_string(self) -> String {
        match self {
            FontOpticalSizing::Auto => "auto",
            FontOpticalSizing::None => "none"
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum FontStyle {
    Normal, Italic, Oblique, ObliqueAngle(Angle)
}

impl AttributeValue for FontStyle {
    fn attr_string(self) -> String {
        match self {
            FontStyle::Normal => "normal".to_string(),
            FontStyle::Italic => "italic".to_string(),
            FontStyle::Oblique => "oblique".to_string(),
            FontStyle::ObliqueAngle(angle) => format!("oblique {}", angle.attr_string()),
        }
    }
}

#[derive(Clone, Copy)]
pub enum JustifyContent {
    FlexStart, FlexEnd, Center, SpaceBetween, SpaceAround, SpaceEvenly
}

impl AttributeValue for JustifyContent {
    fn attr_string(self) -> String {
        match self {
            JustifyContent::FlexStart => "flex-start",
            JustifyContent::FlexEnd => "flex-end",
            JustifyContent::Center => "center",
            JustifyContent::SpaceBetween => "space-between",
            JustifyContent::SpaceAround => "space-around",
            JustifyContent::SpaceEvenly => "space-evenly"
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub struct Length {
    value: f64,
    unit: LengthUnit
}

#[derive(Clone, Copy)]
pub enum LengthUnit {
    Px, Pt, Em, Ch, Rem, Vw, Vh, Percent
}

impl Length {
    pub fn new(value: f64, unit: LengthUnit) -> Self {
        Self { value, unit }
    }
}

pub trait IntoLength {
    fn px(self) -> Length;
    fn pt(self) -> Length;
    fn em(self) -> Length;
    fn ch(self) -> Length;
    fn rem(self) -> Length;
    fn vw(self) -> Length;
    fn vh(self) -> Length;
    fn percent(self) -> Length;
}

impl <T> IntoLength for T where T: Into<f64> {
    fn px(self) -> Length { Length::new(self.into(), LengthUnit::Px) }
    fn pt(self) -> Length { Length::new(self.into(), LengthUnit::Pt) }
    fn em(self) -> Length { Length::new(self.into(), LengthUnit::Em) }
    fn ch(self) -> Length { Length::new(self.into(), LengthUnit::Ch) }
    fn rem(self) -> Length { Length::new(self.into(), LengthUnit::Rem) }
    fn vw(self) -> Length { Length::new(self.into(), LengthUnit::Vw) }
    fn vh(self) -> Length { Length::new(self.into(), LengthUnit::Vh) }
    fn percent(self) -> Length { Length::new(self.into(), LengthUnit::Percent) }
}

impl AttributeValue for Length {
    fn attr_string(self) -> String {
        format!(
            "{}{}",
            self.value,
            match self.unit {
                LengthUnit::Px => "px",
                LengthUnit::Pt => "pt",
                LengthUnit::Em => "em",
                LengthUnit::Ch => "ch",
                LengthUnit::Rem => "rem",
                LengthUnit::Vw => "vw",
                LengthUnit::Vh => "vh",
                LengthUnit::Percent => "%"
            }
        )
    }
}

#[derive(Clone, Copy)]
pub enum Overflow {
    Visible, Hidden, Scroll, Auto
}

impl AttributeValue for Overflow {
    fn attr_string(self) -> String {
        match self {
            Overflow::Visible => "visible",
            Overflow::Hidden => "hidden",
            Overflow::Scroll => "scroll",
            Overflow::Auto => "auto",
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum Position {
    Static, Relative, Fixed, Absolute, Sticky
}

impl AttributeValue for Position {
    fn attr_string(self) -> String {
        match self {
            Position::Static => "static",
            Position::Relative => "relative",
            Position::Fixed => "fixed",
            Position::Absolute => "absolute",
            Position::Sticky => "sticky",
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum TextAlign {
    Start, End, Left, Right, Center, Justify, MatchParent
}

impl AttributeValue for TextAlign {
    fn attr_string(self) -> String {
        match self {
            TextAlign::Start => "start",
            TextAlign::End => "end",
            TextAlign::Left => "left",
            TextAlign::Right => "right",
            TextAlign::Center => "center",
            TextAlign::Justify => "justify",
            TextAlign::MatchParent => "match-parent",
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum TextDecorationStyle {
    Solid, Double, Dotted, Dashed, Wavy
}

impl AttributeValue for TextDecorationStyle {
    fn attr_string(self) -> String {
        match self {
            TextDecorationStyle::Solid => "solid",
            TextDecorationStyle::Double => "double",
            TextDecorationStyle::Dotted => "dotted",
            TextDecorationStyle::Dashed => "dashed",
            TextDecorationStyle::Wavy => "wavy",
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum UserSelect {
    None, Auto, All
}

impl AttributeValue for UserSelect {
    fn attr_string(self) -> String {
        match self {
            UserSelect::None => "none",
            UserSelect::Auto => "auto",
            UserSelect::All => "all",
        }.to_string()
    }
}

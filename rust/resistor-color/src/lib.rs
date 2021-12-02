use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
use serde::Serialize;

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, IntEnum, IntoEnumIterator, PartialEq, Serialize)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => serde_json::to_string(&color)
            .unwrap()
            .trim_matches('"')
            .to_string(),
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}

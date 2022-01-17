use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    //         vec![Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White]
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

impl ResistorColor {
    fn value(color: ResistorColor) -> usize {
        match color {
            ResistorColor::Black => 0,
            ResistorColor::Brown => 1,
            ResistorColor::Red => 2,
            ResistorColor::Orange => 3,
            ResistorColor::Yellow => 4,
            ResistorColor::Green => 5,
            ResistorColor::Blue => 6,
            ResistorColor::Violet => 7,
            ResistorColor::Grey => 8,
            ResistorColor::White => 9,
        }
    }
}

pub fn color_to_value(color: ResistorColor) -> usize {
    ResistorColor::value(color) 
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value as u8) {
        Ok(r) => format!("{:?}", r),
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let resistor_iter = ResistorColor::into_enum_iter();

    resistor_iter.map(|color| color).collect()
}


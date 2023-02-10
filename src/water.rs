#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum Water {
    Ash,
    Blue,
    Brown,
    Cyan,
    Green,
    Lime,
    Olive,
    Orange,
    Pink,
    Purple,
    Red,
    Yellow,
}

pub enum ParseError {
    Empty,
    UnknownColour,
}

impl TryFrom<Option<&&str>> for Water {
    type Error = ParseError;

    fn try_from(v: Option<&&str>) -> Result<Self, Self::Error> {
        v.ok_or(ParseError::Empty)
            .and_then(<&&str as TryInto<Self>>::try_into)
    }
}

impl TryFrom<&&str> for Water {
    type Error = ParseError;

    fn try_from(value: &&str) -> std::result::Result<Self, Self::Error> {
        match *value {
            "a" | "ash" => Ok(Self::Ash),
            "b" | "bl" | "blue" => Ok(Self::Blue),
            "br" | "brown" => Ok(Self::Brown),
            "c" | "cy" | "cyan" => Ok(Self::Cyan),
            "g" | "green" => Ok(Self::Green),
            "l" | "lime" => Ok(Self::Lime),
            "ol" | "olive" => Ok(Self::Olive),
            "o" | "or" => Ok(Self::Orange),
            "p" | "pi" | "pink" => Ok(Self::Pink),
            "pu" | "purple" => Ok(Self::Purple),
            "r" | "red" => Ok(Self::Red),
            "y" | "yellow" => Ok(Self::Yellow),
            _ => Err(ParseError::UnknownColour),
        }
    }
}

impl Water {
    const fn get_colour(self) -> ansi_term::Colour {
        use ansi_term::Colour::RGB;
        match self {
            Self::Blue => RGB(58, 46, 195),
            Self::Brown => RGB(126, 74, 7),
            Self::Cyan => RGB(84, 163, 228),
            Self::Green => RGB(17, 101, 51),
            Self::Ash => RGB(99, 100, 101),
            Self::Lime => RGB(98, 214, 124),
            Self::Olive => RGB(120, 150, 15),
            Self::Orange => RGB(232, 140, 66),
            Self::Pink => RGB(234, 94, 123),
            Self::Purple => RGB(113, 43, 147),
            Self::Red => RGB(197, 42, 35),
            Self::Yellow => RGB(241, 217, 87),
        }
    }

    pub fn style(self) -> ansi_term::Style {
        ansi_term::Style::new().on(self.get_colour())
    }
}

impl std::fmt::Display for Water {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.style().paint("   ").fmt(f)
    }
}

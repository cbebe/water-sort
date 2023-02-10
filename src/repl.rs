pub enum Error {
    Message(String),
    InvalidPuzzleSize,
    Usage(Usage),
    InvalidTube(usize),
    InvalidPour(usize, usize),
    InvalidIndex,
    UnrecognizedCommand(String),
    UnknownWaterColour(String),
}

pub enum Usage {
    Init,
    Load,
    Save,
    Unset,
    Set,
    Tube,
    QuickTube,
}

fn usage(u: &Usage, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("usage: ")?;
    match u {
        Usage::Init => f.write_str("init <size>"),
        Usage::Load => f.write_str("load <file>"),
        Usage::Save => f.write_str("save <file>"),
        Usage::Unset => f.write_str("unset <tube> <idx>"),
        Usage::Set => f.write_str("set <tube> <idx> <colour>"),
        Usage::Tube => f.write_str("tube [<tube> <colours>]+"),
        Usage::QuickTube => f.write_str("tt [<idx>[<colour>]+]+"),
    }
}

macro_rules! fprintf {
    ($f:ident, $fmt: expr) => {
        $f.write_fmt(format_args!($fmt))
    };
    ($f:ident, $fmt: expr, $($args: tt)*) => {
        $f.write_fmt(format_args!($fmt, $($args)*))
    };
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(m) => f.write_str(m),
            Self::InvalidPuzzleSize => f.write_str("size must be greater than 2"),
            Self::Usage(u) => usage(u, f),
            Self::InvalidTube(size) => fprintf!(f, "tube must be between 0 and {}", size - 1),
            Self::InvalidPour(a, b) => fprintf!(f, "cannot pour from {a} to {b}"),
            Self::InvalidIndex => f.write_str("index must be between 0 and 3"),
            Self::UnrecognizedCommand(c) => fprintf!(f, "Unrecognized command: {c}"),
            Self::UnknownWaterColour(c) => fprintf!(f, "Unknown colour: {c}"),
        }
    }
}

impl Error {
    // e is a String
    #[allow(clippy::nursery::missing_const_for_fn)]
    pub fn from_water(value: crate::water::ParseError, usage: Usage) -> Self {
        match value {
            crate::water::ParseError::Empty => Self::Usage(usage),
            crate::water::ParseError::UnknownColour(e) => Self::UnknownWaterColour(e),
        }
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self::Message(value.to_owned())
    }
}

impl From<&String> for Error {
    fn from(value: &String) -> Self {
        Self::Message(value.clone())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Message(format!("serialization error: {value}"))
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Message(format!("io error: {value}"))
    }
}

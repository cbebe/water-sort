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
        Usage::Init => write!(f, "init <size>"),
        Usage::Load => write!(f, "load <file>"),
        Usage::Save => write!(f, "save <file>"),
        Usage::Unset => write!(f, "unset <tube> <idx>"),
        Usage::Set => write!(f, "set <tube> <idx> <colour>"),
        Usage::Tube => write!(f, "tube [<tube> <colours>]+"),
        Usage::QuickTube => write!(f, "tt [<idx>[<colour>]+]+"),
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(m) => write!(f, "{m}"),
            Self::InvalidPuzzleSize => write!(f, "size must be greater than 2"),
            Self::Usage(u) => usage(u, f),
            Self::InvalidTube(size) => write!(f, "tube must be between 0 and {}", size - 1),
            Self::InvalidPour(a, b) => write!(f, "cannot pour from {a} to {b}"),
            Self::InvalidIndex => write!(f, "index must be between 0 and 3"),
            Self::UnrecognizedCommand(c) => write!(f, "Unrecognized command: {c}"),
            Self::UnknownWaterColour(c) => write!(f, "Unknown colour: {c}"),
        }
    }
}

impl Error {
    // e is a String
    #[allow(clippy::missing_const_for_fn)]
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

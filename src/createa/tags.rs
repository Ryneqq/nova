use std::str::FromStr;

const IMPORT:       &str = "[IMPORT]";
const TYPE:         &str = "[TYPE]";
const INITIALIZE:   &str = "[INITIALIZE]";
const IMPLEMENT:    &str = "[IMPLEMENT]";
const CONVERT:      &str = "[CONVERT]";
const TRY_CONVERT:  &str = "[TRY.CONVERT]";
const ITERATE:      &str = "[ITERATE]";
const DEREFERENCE:  &str = "[DEREFERENCE]";
const DISPLAY:      &str = "[DISPLAY]";
const TESTS:        &str = "[TESTS]";

pub enum Tags {
    Import,
    Type,
    Initialize,
    Implement,
    Convert,
    TryConvert,
    Iterate,
    Dereference,
    Display,
    Tests,
}

impl FromStr for Tags {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            IMPORT      => Ok(Tags::Import),
            TYPE        => Ok(Tags::Type),
            INITIALIZE  => Ok(Tags::Initialize),
            IMPLEMENT   => Ok(Tags::Implement),
            CONVERT     => Ok(Tags::Convert),
            TRY_CONVERT => Ok(Tags::TryConvert),
            ITERATE     => Ok(Tags::Iterate),
            DEREFERENCE => Ok(Tags::Dereference),
            DISPLAY     => Ok(Tags::Display),
            TESTS       => Ok(Tags::Tests),
            _           => Err(String::from("Option not known"))
        }
    }
}

impl AsRef<str> for Tags {
    fn as_ref(&self) -> &str {
        match *self {
            Tags::Import        => IMPORT,
            Tags::Type          => TYPE,
            Tags::Initialize    => INITIALIZE,
            Tags::Implement     => IMPLEMENT,
            Tags::Convert       => CONVERT,
            Tags::TryConvert    => TRY_CONVERT,
            Tags::Iterate       => ITERATE,
            Tags::Dereference   => DEREFERENCE,
            Tags::Display       => DISPLAY,
            Tags::Tests         => TESTS,
        }
    }
}

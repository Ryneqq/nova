pub const IMPORT_DEREF: &str = r#"
use std::ops::Deref;
"#;

pub const IMPORT_TRY_CONVERT: &str = r#"
use std::convert::{TryFrom, TryInto};
"#;

pub const TYPE: &str = r#"
type Owl = ((),());
"#;

// ======== BASIC STRUCT ========
pub const INITIALIZE_STRUCT: &str = r#"
#[derive(Debug, Clone)]
pub struct _n_ {}
"#;

pub const IMPLEMENT_STRUCT: &str = r#"
impl _n_ {
    pub fn new() -> Self {
        Self {}
    }
}
"#;
// ==============================

// ========= BASIC ENUM =========
pub const INITIALIZE_ENUM: &str = r#"
#[derive(Debug, Clone)]
pub enum _n_ {
    EmptyVariant,
    TupleVariant(),
    StructVariant{},
}
"#;
// ==============================

/// Depends on: [TYPE]
pub const CONVERT: &str = r#"
impl From<Owl> for _n_ {
    fn from(f: Owl) -> Self {
        unimplemented!()
    }
}

impl Into<Owl> for _n_ {
    fn into(self) -> Owl {
        unimplemented!()
    }
}
"#;

/// Depends on: [TYPE] & [IMPORT_TRY_CONVERT]
pub const TRY_CONVERT: &str = r#"
impl TryFrom<Owl> for _n_ {
    type Error = Owl;

    fn try_from(value: Owl) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

impl TryInto<Owl> for _n_ {
    type Error = Owl;

    fn try_into(self) -> Result<Owl, Self::Error> {
        unimplemented!()
    }
}
"#;

/// Depends on: [TYPE]
pub const ITERATE: &str = r#"
impl Iterator for _n_ {
    type Item = Owl;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
"#;

/// Depends on: [TYPE] & [IMPORT_DEREF]
pub const DEREFERENCE: &str = r#"
impl Deref for _n_ {
    type Target = Owl;

    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}
"#;

pub const TESTS: &str = r#"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        unimplemented!();
    }
}
"#;

pub const BASIC_STRUCT_DATA: &[&str]        = &[INITIALIZE_STRUCT, IMPLEMENT_STRUCT, TESTS];
pub const CONVERT_STRUCT_DATA: &[&str]      = &[TYPE, INITIALIZE_STRUCT, IMPLEMENT_STRUCT, CONVERT, TESTS];
pub const TRY_CONVERT_STRUCT_DATA: &[&str]  = &[IMPORT_TRY_CONVERT, TYPE, INITIALIZE_STRUCT, IMPLEMENT_STRUCT, TRY_CONVERT, TESTS];
pub const FULL_STRUCT_DATA: &[&str]         = &[IMPORT_TRY_CONVERT, IMPORT_DEREF, TYPE, INITIALIZE_STRUCT, IMPLEMENT_STRUCT, CONVERT, DEREFERENCE, ITERATE, TESTS];

pub const BASIC_ENUM_DATA: &[&str]          = &[INITIALIZE_ENUM, TESTS];
pub const CONVERT_ENUM_DATA: &[&str]        = &[TYPE, INITIALIZE_ENUM, CONVERT, TESTS];
pub const TRY_CONVERT_ENUM_DATA: &[&str]    = &[IMPORT_TRY_CONVERT, TYPE, INITIALIZE_ENUM, TRY_CONVERT, TESTS];
pub const FULL_ENUM_DATA: &[&str]           = &[IMPORT_TRY_CONVERT, IMPORT_DEREF, TYPE, INITIALIZE_ENUM, CONVERT, DEREFERENCE, ITERATE, TESTS];


pub fn generate_struct(name: &str) -> String {
    format!(r#"
#[derive(Debug, Clone)]
pub struct {} {{}}

impl {} {{
    pub fn new() -> Self {{
        Self {{}}
    }}
}}

type Owl = ((),());
impl From<Owl> for {} {{
    fn from(f: Owl) -> Self {{
        unimplemented!()
    }}
}}

mod tests {{
    use super::*;

    #[test]
    fn name() {{
        unimplemented!();
    }}
}}
    "#, name, name, name )
}

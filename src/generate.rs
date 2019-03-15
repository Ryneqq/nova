pub enum Generate<'a> {
    Struct(&'a str),
    Enum(&'a str),
}

impl<'a> Generate<'a> {
    pub fn build(&self) -> String {
        use Generate::*;

        match self {
            Enum(name)      => generate_enum(name),
            Struct(name)    => generate_struct(name),
        }
    }
}

fn generate_struct(name: &str) -> String {
    let camel_case = to_camel_case(name);
    let name = &camel_case;

    format!(r#"#[derive(Debug)]
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
}}"#, name, name, name )
}

fn generate_enum(name: &str) -> String {
    let camel_case = to_camel_case(name);
    let name = &camel_case;

    format!(r#"type Owl = ((),());

#[derive(Debug)]
pub enum {} {{
    EmptyVariant,
    TupleVariant(Owl),
    StructVariant{{ owl: Owl }},
}}

impl {} {{}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn name() {{
        unimplemented!();
    }}
}}"#, name, name)
}

fn to_camel_case(name: &str) -> String {
    let mut retval = String::new();

    name.to_string()
        .split('_')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let f = s.chars()
                .next()
                .unwrap()
                .to_uppercase()
                .to_string();
            let mut s = s.to_string();
            s.replace_range(0..1, &f);

            s
        }).for_each(|s| {
            retval.push_str(&s);
        });

    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converting_to_camel_case() {
        assert_eq!("ToCamelCase", to_camel_case("ToCamelCase"));
        assert_eq!("ToCamelCase", to_camel_case("to_camel_case"));
        assert_eq!("ToCamelCase", to_camel_case("To_Camel_Case"));
        assert_eq!("ToCamelCase", to_camel_case("_to_camel_case_"));
        assert_eq!("ToCamelCase", to_camel_case("_toCamelCase"));
        assert_eq!("",            to_camel_case("_____________"));
    }
}

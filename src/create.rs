use itertools::Itertools;


fn to_snake_case(name: &str) -> String {
    let mut name = name.to_string();

    match name.contains("_") {
        true  => name.to_lowercase(),
        false => {
            name.chars()
                .map(|c| match c.is_uppercase() {
                    false => c.to_string(),
                    true  => format!("_{}", c.to_lowercase())
                })
                .join("")
                .chars()
                .skip_while(|c| *c == '_')
                .join("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converting_to_snake_case() {
        assert_eq!("to_snake_case",         to_snake_case("to_snake_case"));
        assert_eq!("to_snake_case",         to_snake_case("To_Snake_Case"));
        assert_eq!("to_snake_case",         to_snake_case("ToSnakeCase"));
        assert_eq!("to_snake_case",         to_snake_case("toSnakeCase"));
        assert_eq!("t_o_s_n_a_k_e_c_a_s_e", to_snake_case("TOSNAKECASE"));
        assert_eq!("t_o_s_n_a_k_e_c_a_s_e", to_snake_case("tOSNAKECASE"));
        assert_eq!("_tosnakecase",          to_snake_case("_TOSNAKECASE"));
        assert_eq!("_____________",         to_snake_case("_____________"));
    }
}

type PlaceHolder = ();

#[derive(Debug, Clone)]
pub enum _n_ {
    EmptyVariant,
    TupleVariant(PlaceHolder),
    StructVariant{ field: PlaceHolder },
}

impl _n_ {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        unimplemented!();
    }
}

pub enum Case {
    Snake,
    Camel,
    Pascal,
}

pub trait StrChoice {
    /// Convert the implementing type to a string in the specified case
    fn to_str(&self, case: Case) -> String;
}

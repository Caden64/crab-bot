use edres::{generate_enum, EnumOptions, Options, ValuesStructOptions};
use std::borrow::Cow;

fn main() {
    let mut options = Options::minimal();
    options.enums = EnumOptions {
        derived_traits: Cow::Borrowed(&[
            Cow::Borrowed("Debug"),
            Cow::Borrowed("Clone"),
            Cow::Borrowed("Copy"),
            Cow::Borrowed("PartialEq"),
            Cow::Borrowed("Eq"),
            Cow::Borrowed("Hash"),
            Cow::Borrowed("poise::ChoiceParameter"),
        ]),
        impl_default: true,
        impl_display: true,
        impl_from_str: true,
        all_variants_const_name: Some(Cow::Borrowed("ALL")),
        all_values_const_name: Some(Cow::Borrowed("VALUES")),
        values_struct: Some(ValuesStructOptions::new()),
        get_value_fn_name: Some(Cow::Borrowed("get")),
    };
    generate_enum("src/roles.yaml", "Roles", &Options::minimal()).unwrap();
}
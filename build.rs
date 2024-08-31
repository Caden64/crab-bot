use config_struct::{EnumOptions, Error};
fn main() -> Result<(), Error>{
    /*
    config_struct::create_enum(
        "roles.yaml",
        "src/roles.rs",
        &EnumOptions {
            format: None,
            enum_name: "Roles".to_string(),
            all_variants_const: None,
            derived_traits: vec!["Debug", "Clone", "Copy", "PartialEq", "Eq", "PartialOrd", "Ord", "Hash", "poise::ChoiceParameter"].iter().map(|x| x.to_string()).collect(),
            first_variant_is_default: true,
            impl_display: true,
            impl_from_str: true,
            serde_support: Default::default(),
            use_serde_derive_crate: false,
            create_dirs: true,
            write_only_if_changed: true
        })
     */
    Ok(())
}
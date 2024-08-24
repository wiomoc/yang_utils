use crate::errors::ErrorContext;
use crate::parser::model::Type;
use crate::typing::{collect_unused_fields, TypeTrait};

pub(crate) struct BoolType {}

impl TypeTrait for BoolType {
    fn get_rust_type(&self) -> &'static str {
        "bool"
    }

    fn derive(&self, model: &Type, error_context: &mut ErrorContext) -> Result<Self, ()> {
        let unused_fields = collect_unused_fields!(
            model,
            [
                base,
                bit,
                r#enum,
                fraction_digits,
                length,
                range,
                path,
                pattern,
                require_instance,
                r#type
            ]
        );

        if !unused_fields.is_empty() {
            error_context.add_warning((0, 0), format!("Unused fields: {:?}", unused_fields));
        }
        Ok(Self {})
    }
}

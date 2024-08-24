use crate::errors::ErrorContext;
use crate::parser::model::Type;
use crate::typing::{collect_unused_fields, TypeTrait};
use std::cmp::max;
use std::collections::HashSet;

pub(crate) struct EnumerationType {
    pub(crate) items: Option<Vec<(String, i32)>>,
}

impl TypeTrait for EnumerationType {
    fn get_rust_type(&self) -> &'static str {
        "i32"
    }

    fn derive(&self, model: &Type, error_context: &mut ErrorContext) -> Result<Self, ()> {
        let unused_fields = collect_unused_fields!(
            model,
            [
                base,
                bit,
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

        let is_subtype = !self.is_abstract_type();

        if HashSet::<&str>::from_iter(model.r#enum.iter().map(|item| item.name.as_str())).len()
            != model.r#enum.len()
        {
            error_context.add_error((0, 0), "Duplicate enum item names".to_string());
            return Err(());
        }
        let mut items = Vec::with_capacity(model.r#enum.len());
        if !is_subtype {
            let mut max_value = -1;
            for r#enum in model.r#enum.iter() {
                let value = if let Some(value) = r#enum.value {
                    if items.iter().any(|(_, v)| *v == value) {
                        error_context.add_error(
                            (0, 0),
                            format!("Duplicate value {} for enum item {}", value, r#enum.name),
                        );
                        return Err(());
                    }
                    max_value = max(max_value, value);
                    value
                } else {
                    max_value += 1;
                    max_value
                };
                items.push((r#enum.name.clone(), value));
            }
        } else {
            for r#enum in model.r#enum.iter() {
                if !self
                    .items
                    .as_ref()
                    .map(|items| items.as_slice())
                    .unwrap_or(&[])
                    .iter()
                    .find(|(name, value)| {
                        name == &r#enum.name
                            && r#enum
                                .value
                                .map(|base_value| base_value == *value)
                                .unwrap_or(true)
                    })
                    .is_some()
                {
                    error_context
                        .add_error((0, 0), format!("Enum not in base enum {}", r#enum.name));
                    return Err(());
                }
                items.push((r#enum.name.clone(), 0));
            }
        };

        Ok(EnumerationType { items: Some(items) })
    }

    fn is_abstract_type(&self) -> bool {
        self.items.is_none()
    }
}

use regex::RegexSet;
use crate::errors::ErrorContext;
use crate::parser::model;
use crate::typing::range::{derive_range, IntRangePart};
use crate::typing::{collect_unused_fields, SingleItemOrVec, TypeTrait};

#[derive(Debug, Clone)]
pub(crate) struct StringType {
    pub(crate) length: SingleItemOrVec<IntRangePart>,
    pub(crate) pattern: Option<RegexSet>,
}


impl TypeTrait for StringType {
    fn get_rust_type(&self) -> &'static str {
        "String"
    }

    fn derive(&self, model: &model::Type, error_context: &mut ErrorContext) -> Result<Self, ()> {
        let unused_fields = collect_unused_fields!(
            model,
            [
                base,
                bit,
                r#enum,
                fraction_digits,
                range,
                path,
                require_instance,
                r#type
            ]
        );

        if !unused_fields.is_empty() {
            error_context.add_warning((0, 0), format!("Unused fields: {:?}", unused_fields));
        }

        let length = derive_range(
            &self.length,
            model
                .length
                .as_ref()
                .map(|length| &length.length_expression),
            error_context,
        )?;

        let pattern = if self.pattern.is_some() && model.pattern.is_empty() {
            self.pattern.clone()
        } else if self.pattern.is_some() || !model.pattern.is_empty() {
            let base_pattern = self
                .pattern
                .as_ref()
                .map(|pattern| pattern.patterns())
                .unwrap_or(&[])
                .iter()
                .chain(model.pattern.iter().map(|pattern| &pattern.regex));
            Some(RegexSet::new(base_pattern).map_err(|e| {
                error_context.add_error((0, 0), e.to_string());
                ()
            })?)
        } else {
            None
        };

        Ok(StringType { length, pattern })
    }
}
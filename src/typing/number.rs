use crate::errors::ErrorContext;
use crate::parser::model;
use crate::typing::range::{derive_range, DecimalRangePart, IntRangePart, RangePart};
use crate::typing::{collect_unused_fields, SingleItemOrVec, TypeTrait};

#[derive(Debug, Clone)]
pub(crate) struct IntType {
    pub(crate) range: SingleItemOrVec<IntRangePart>,
    pub(crate) rust_type: &'static str,
}

impl TypeTrait for IntType {
    fn get_rust_type(&self) -> &'static str {
        self.rust_type
    }

    fn derive(&self, model: &model::Type, error_context: &mut ErrorContext) -> Result<Self, ()> {
        let unused_fields = collect_unused_fields!(
            model,
            [
                base,
                bit,
                r#enum,
                fraction_digits,
                length,
                path,
                pattern,
                require_instance,
                r#type
            ]
        );

        if !unused_fields.is_empty() {
            error_context.add_warning((0, 0), format!("Unused fields: {:?}", unused_fields));
        }

        let range = derive_range(
            &self.range,
            model.range.as_ref().map(|range| &range.range_expression),
            error_context,
        )?;

        Ok(IntType {
            range,
            rust_type: self.rust_type,
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DecimalType {
    pub(crate) range: Option<SingleItemOrVec<DecimalRangePart>>,
    pub(crate) fraction_digits: Option<u8>,
}

impl TypeTrait for DecimalType {
    fn get_rust_type(&self) -> &'static str {
        "f64"
    }

    fn derive(&self, model: &model::Type, error_context: &mut ErrorContext) -> Result<Self, ()> {
        let unused_fields = collect_unused_fields!(
            model,
            [
                base,
                bit,
                r#enum,
                length,
                path,
                pattern,
                require_instance,
                r#type
            ]
        );

        if !unused_fields.is_empty() {
            error_context.add_warning((0, 0), format!("Unused fields: {:?}", unused_fields));
        }

        let fraction_digits = model
            .fraction_digits
            .map(|fraction_digits| {
                if fraction_digits < 0 || fraction_digits > 18 {
                    error_context.add_error(
                        (0, 0),
                        "Fraction digits must be between 0 and 18".to_string(),
                    );
                    return Err(());
                }
                Ok(fraction_digits as u8)
            })
            .transpose()?
            .or(self.fraction_digits);

        let fraction_range = fraction_digits.map(|fraction_digits| {
            SingleItemOrVec::Single(RangePart {
                min_value: 10usize.pow(fraction_digits as u32 - 18) as f64 * 9.223372036854775808,
                max_value: 10usize.pow(fraction_digits as u32 - 18) as f64 * -9.223372036854775808,
            })
        });

        let range = self
            .range
            .as_ref()
            .or(fraction_range.as_ref())
            .map(|range| {
                derive_range(
                    range,
                    model.range.as_ref().map(|range| &range.range_expression),
                    error_context,
                )
            })
            .transpose()?;

        Ok(DecimalType {
            range,
            fraction_digits,
        })
    }

    fn is_abstract_type(&self) -> bool {
        self.range.is_none() || self.fraction_digits.is_none()
    }
}

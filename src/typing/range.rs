use std::ops::Add;
use std::str::FromStr;
use crate::errors::ErrorContext;
use crate::parser::model;
use crate::parser::model::{LengthBoundary, RangeBoundary};
use crate::typing::SingleItemOrVec;

pub (crate) type IntRangePart = RangePart<i128>;
pub (crate) type DecimalRangePart = RangePart<f64>;

#[derive(Debug, Clone)]
pub (crate) struct RangePart<T> {
    pub(crate) min_value: T,
    pub(crate) max_value: T,
}

trait RangeElement: PartialOrd + Copy {
    fn try_from_i128(value: i128) -> Option<Self>
    where
        Self: Sized;

    fn try_from_f64(value: f64) -> Option<Self>
    where
        Self: Sized;

    fn next_value(&self) -> Self;
}

impl RangeElement for i128 {
    fn try_from_i128(value: i128) -> Option<Self> {
        Some(value)
    }

    fn try_from_f64(_value: f64) -> Option<Self> {
        None
    }

    fn next_value(&self) -> Self {
        self.add(1)
    }
}

impl RangeElement for f64 {
    fn try_from_i128(value: i128) -> Option<Self> {
        Some(value as f64)
    }

    fn try_from_f64(value: f64) -> Option<Self> {
        Some(value)
    }

    fn next_value(&self) -> Self {
        *self
    }
}

trait ResolveBoundary<T: RangeElement>: Copy + FromStr<Err = String> {
    fn resolve(boundary: Self, base_range: &SingleItemOrVec<RangePart<T>>) -> Result<T, String>;

    fn is_min(&self) -> bool;

    fn is_max(&self) -> bool;
}

impl<T: RangeElement> ResolveBoundary<T> for RangeBoundary {
    fn resolve(
        boundary: RangeBoundary,
        base_range: &SingleItemOrVec<RangePart<T>>,
    ) -> Result<T, String> {
        Ok(match boundary {
            RangeBoundary::Min => base_range.iter().next().unwrap().min_value,
            RangeBoundary::Max => base_range.iter().last().unwrap().max_value,
            RangeBoundary::Integer(i) => T::try_from_i128(i as i128).unwrap(),
            RangeBoundary::Decimal(d) => {
                if let Some(d) = T::try_from_f64(d) {
                    d
                } else {
                    return Err("Decimal is not allowed in integer type".to_string());
                }
            }
        })
    }

    fn is_min(&self) -> bool {
        matches!(self, Self::Min)
    }

    fn is_max(&self) -> bool {
        matches!(self, Self::Max)
    }
}

impl<T: RangeElement> ResolveBoundary<T> for LengthBoundary {
    fn resolve(
        boundary: LengthBoundary,
        base_range: &SingleItemOrVec<RangePart<T>>,
    ) -> Result<T, String> {
        Ok(match boundary {
            LengthBoundary::Min => base_range.iter().next().unwrap().min_value,
            LengthBoundary::Max => base_range.iter().last().unwrap().max_value,
            LengthBoundary::Value(i) => T::try_from_i128(i as i128).unwrap(),
        })
    }

    fn is_min(&self) -> bool {
        matches!(self, Self::Min)
    }

    fn is_max(&self) -> bool {
        matches!(self, Self::Max)
    }
}

pub (crate) fn derive_range<T: RangeElement, B: ResolveBoundary<T>>(
    base_range: &SingleItemOrVec<RangePart<T>>,
    model_range: Option<&model::LengthRangePattern<B>>,
    error_context: &mut ErrorContext,
) -> Result<SingleItemOrVec<RangePart<T>>, ()> {
    let mut last_range_part: Option<(T, T)> = None;
    Ok(if let Some(range) = model_range.as_ref() {
        let mut model_range_iter = range.0.iter().enumerate();

        let mut new_range = vec![];
        for (pos, model_range) in model_range_iter {
            let model_upper_boundary = model_range
                .upper_boundary
                .unwrap_or(model_range.lower_boundary);

            if pos != 0 && model_range.lower_boundary.is_min() {
                error_context.add_error((0, 0), "Min can only be used in first part".to_string());
                return Err(());
            }

            if pos != range.0.len() - 1 && model_range.lower_boundary.is_max() {
                error_context.add_error((0, 0), "Max can only be used in last part".to_string());
                return Err(());
            }

            let min = B::resolve(model_range.lower_boundary, base_range).map_err(|e| {
                error_context.add_error((0, 0), e);
                ()
            })?;
            let max = B::resolve(model_upper_boundary, base_range).map_err(|e| {
                error_context.add_error((0, 0), e);
                ()
            })?;

            if min > max {
                error_context.add_error((0, 0), "Min value is larger than max value".to_string());
                return Err(());
            }

            if let Some((_last_min, last_max)) = last_range_part {
                if last_max > min {
                    error_context.add_error((0, 0), "Range parts not ascending sorted".to_string());
                    return Err(());
                }
            }

            let matched_range = base_range
                .iter()
                .find(|range_part| range_part.min_value <= min && range_part.max_value >= max)
                .is_some();

            if matched_range {
                if let Some((last_min, _last_max)) =
                    last_range_part.filter(|(_last_min, last_max)| T::next_value(last_max) == min)
                {
                    new_range.pop();
                    new_range.push(RangePart::<T> {
                        min_value: last_min,
                        max_value: max,
                    });
                } else {
                    new_range.push(RangePart::<T> {
                        min_value: min,
                        max_value: max,
                    });
                }
            } else {
                error_context.add_error(
                    (0, 0),
                    "Range is larger than range of base type".to_string(),
                );
            }
            last_range_part = Some((min, max));
        }
        SingleItemOrVec::Vec(new_range)
    } else {
        base_range.clone()
    })
}
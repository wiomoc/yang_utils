use crate::parser::parser::Statement;
use crate::parser::lexer::Loc;

pub(crate) type Result<T> = std::result::Result<T, ()>;

#[derive(Debug, Default)]
pub struct ErrorContext {
    errors: Vec<(Loc, String)>,
    warnings: Vec<(Loc, String)>,
}

impl ErrorContext {
   pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add_error(&mut self, loc: Loc, error: String) {
        self.errors.push((loc, error));
    }

    pub(crate) fn add_warning(&mut self, loc: Loc, warning: String) {
        self.warnings.push((loc, warning));
    }

    pub fn print_errors(&self) {
        for (loc, error) in &self.errors {
            eprintln!("Error at {}: {}", loc, error);
        }
    }

    pub fn print_warnings(&self) {
        for (loc, warning) in &self.warnings {
            eprintln!("Warning at {}: {}", loc, warning);
        }
    }
}

pub(crate) trait Mapper<T> {
    fn map(statement: Statement, error_context: &mut ErrorContext) -> Result<T>;
}

pub(crate) trait ArgumentMapper<T> {
    fn map_argument(argument: String, argument_loc: Loc, error_context: &mut ErrorContext) -> Result<T>;
}

impl ArgumentMapper<String> for String {
    fn map_argument(argument: String, _argument_loc: Loc, _error_context: &mut ErrorContext) -> Result<String> {
        Ok(argument)
    }
}


impl Mapper<String> for String {
    fn map(statement: Statement, error_context: &mut ErrorContext) -> Result<String> {
        if !statement.statements.is_empty() {
            error_context.add_warning(statement.keyword_loc.0, "Unexpected statements".to_string());
        }
        match statement.argument {
            Some(arg) => Ok(arg),
            None => {
                error_context.add_error(statement.argument_loc.0, "Expected argument".to_string());
                Err(())
            },
        }
    }
}

macro_rules! prioritize_name {
    ($ident:ident) => {
        stringify!($ident)
    };
     ($name:literal, $ident:ident) => {
        $name
    };
}

macro_rules! model {
    ($keyword:pat, $struc:ident,
    $($argument_ident:ident : $(One<$argument_type_one:ty>)? $(Option<$argument_type_optional:ty>)?,)?
    {
        $(
            $attribute_ident:ident : $(One<$attribute_type_one:ident>)? $(Option<$attribute_type_optional:ident>)? $(Vec<$attribute_type_multiple:ident>)? $(=> $attribute_name:literal)?
        ),*
        $(,)?
    }
    ) => {
         #[derive(Debug)]
         pub struct $struc {
            $(
                pub $argument_ident: $($argument_type_one)? $(Option<$argument_type_optional>)?,
            )?
            $(
                $(
                     pub $attribute_ident: $attribute_type_one,
                )?
                $(
                     pub $attribute_ident: Option<$attribute_type_optional>,
                )?
                $(
                     pub $attribute_ident: Vec<$attribute_type_multiple>,
                )?
            )*
         }
        impl $crate::parser::model_mapper::Mapper<$struc> for $struc {
            fn map(statement: $crate::parser::parser::Statement, error_context: &mut ErrorContext) -> $crate::parser::model_mapper::Result<$struc> {
                if !matches!(statement.keyword.as_str(), $keyword) {
                    error_context.add_error(statement.argument_loc.0, format!("Expected {} keyword", stringify!($keyword)));

                    return Err(());
                }

                let mut error_occured = false;

                $(
                    $(
                        let $argument_ident:Option<$argument_type_one> = if let Some(argument) = statement.argument {
                            if let Ok(argument) = <$argument_type_one>::map_argument(
                                argument,
                                statement.argument_loc.0,
                                error_context
                            ) {
                                Some(argument)
                            } else {
                                error_occured = true;
                                None
                            }
                        } else {
                            error_context.add_error(statement.argument_loc.0, format!("Expected {} argument", stringify!($argument_ident)));
                            error_occured = true;
                            None
                        };
                    )?

                    $(
                    let $argument_ident:Option<$argument_type_optional> = if let Some(argument) = statement.argument {
                            if let Ok(argument) = <$argument_type_optional>::map_argument(
                                argument,
                                statement.argument_loc.0,
                                error_context
                            ) {
                                Some(argument)
                            } else {
                                error_occured = true;
                                None
                            }
                        } else {
                            None
                        };
                    )?
                )?

                $(
                    $(
                        let mut $attribute_ident: Option<$attribute_type_one> = None;
                    )?
                    $(
                        let mut $attribute_ident: Option<$attribute_type_optional> = None;
                    )?
                    $(
                        let mut $attribute_ident: Vec<$attribute_type_multiple> = Vec::new();
                    )?
                )*

                for statement in statement.statements {
                    match statement.keyword.as_str() {
                        $($crate::parser::model_mapper::prioritize_name!($($attribute_name,)? $attribute_ident) => {
                            $(
                                if $attribute_ident.is_some() {
                                    error_context.add_error(statement.keyword_loc.0, format!("Unexpected multiple {}", stringify!($attribute_ident)));
                                    error_occured = true;
                                    continue;
                                }
                                if let Ok(att) = $attribute_type_one::map(statement, error_context) {
                                    $attribute_ident = Some(att);
                                } else {
                                    error_occured = true;
                                }
                            )?
                            $(
                                if $attribute_ident.is_some() {
                                    error_context.add_error(statement.keyword_loc.0, format!("Unexpected multiple {}", stringify!($attribute_ident)));
                                    error_occured = true;
                                    continue;
                                }
                                if let Ok(att) = $attribute_type_optional::map(statement, error_context) {
                                    $attribute_ident = Some(att);
                                } else {
                                    error_occured = true;
                                }
                            )?
                            $(
                                if let Ok(att) = $attribute_type_multiple::map(statement, error_context) {
                                     $attribute_ident.push(att);
                                } else {
                                    error_occured = true;
                                }
                            )?
                        }
                    )*
                        _ => {
                            error_context.add_warning(statement.keyword_loc.0, format!("Unexpected keyword {}", statement.keyword));
                        }
                    }
                }

                $(
                    let _att_name = $crate::parser::model_mapper::prioritize_name!($($attribute_name,)? $attribute_ident);
                    $(
                        let _dummy: $attribute_type_one;
                        if $attribute_ident.is_none() {
                            error_context.add_error(statement.keyword_loc.0, format!("Expected {} attribute", _att_name));
                            error_occured = true;
                        }
                    )?
                )*

                if error_occured {
                    return Err(());
                }

                $(
                    $(
                        let $argument_ident:$argument_type_one = $argument_ident.unwrap();
                     )?
                )?

                $(
                    $(
                        let $attribute_ident: $attribute_type_one = $attribute_ident.unwrap();
                    )?
                )*


                let inst = $struc {
                    $(
                        $argument_ident,
                    )?
                    $(
                        $attribute_ident
                    ),*

                };
                Ok(inst)
            }
        }
    };
}

pub(crate) use model;
pub(crate) use prioritize_name;

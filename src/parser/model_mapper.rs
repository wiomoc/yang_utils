
use crate::parser::parser::Statement;

type Result<T> = std::result::Result<T, String>;

trait Mapper<T> {
    fn map(statement: Statement) -> Result<T>;
}

impl Mapper<String> for String {
    fn map(statement: Statement) -> Result<String> {
        if !statement.statements.is_empty() {
            return Err("Expected string".to_string());
        }
        statement.argument.ok_or("Expected string".to_string())
    }
}

macro_rules! module {
    ($keyword:ident, $struc:ident,
    $(argument: $argument:ident,)?
    $(opt_argument: $opt_argument:ident,)?
    {
        $(
            $attribute_name:ident : $(One<$type_one:ident>)? $(Option<$type_optional:ident>)? $(Vec<$type_multiple:ident>)?
        ),*
    }) => {
         pub struct $struc {
            $(
                $argument: String,
            )?
            $(
                $opt_argument: Option<String>,
            )?
            $(
                $(
                     pub $attribute_name: $type_one,
                )?
                $(
                     pub $attribute_name: Option<$type_optional>,
                )?
                $(
                     pub $attribute_name: Vec<$type_multiple>,
                )?
            )*
        }
        impl Mapper<$struc> for $struc {
            fn $map(statement: Statement) -> Result<$struc> {
                if statement.keyword.as_str() != stringify!($keyword) {
                    return Err(format!("Expected {} keyword", stringify!($keyword)));
                }

                $(
                    let $argument = statement.argument.ok_or(format!("Expected {} argument", stringify!($argument)))?;
                )?

                $(
                    let $opt_argument = statement.argument;
                )?

                $(
                    $(
                        let mut $attribute_name: Option<$type_one> = None;
                    )?
                    $(
                        let mut $attribute_name: Option<$type_optional> = None;
                    )?
                    $(
                        let mut $attribute_name: Vec<$type_multiple> = Vec::new();
                    )?
                )*

                for statement in statement.statements {
                    match statement.keyword.as_str() {
                        $(stringify!($attribute_name) => {
                            $(
                                if $attribute_name.is_some() {
                                    return Err(format!("Unexpected multiple {}", stringify!($attribute_name)));
                                }
                                let att: Option<$type_one> = $type_one::map(statement)?;
                                $attribute_name = att;
                            )?
                            $(
                                if $attribute_name.is_some() {
                                    return Err(format!("Unexpected multiple {}", stringify!($attribute_name)));
                                }
                                let att: Option<$type_optional> = $type_optional::map(statement)?;
                                $attribute_name = att;
                            )?
                            $(
                                let att: Option<$type_multiple> = $type_multiple::map(statement)?;
                                $attribute_name.push(att);
                            )?
                        }
                    )*
                        _ => return Err(format!("Unexpected keyword: {}", statement.keyword)),
                    }
                }

                $(
                    $(
                        let $attribute_name: $type_one  = $attribute_name.ok_or(format!("Expected {} attribute", stringify!($attribute_name)))?;
                    )?
                )*

                let $keyword = $struc {
                    $(
                        $argument,
                    )?
                    $(
                        $opt_argument,
                    )?
                    $(
                        $attribute_name
                    ),*
                };
                Ok($keyword)
            }
        }
    };
}

module! {
    import, Import,
    opt_argument: module,
    {
        description: Option<String>,
        prefix: One<String>,
        reference: Vec<String>
    }
}
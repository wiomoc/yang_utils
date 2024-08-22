use crate::parser::model_mapper::{model, ArgumentMapper, ErrorContext, Mapper};
use crate::parser::parser::Statement;
use std::str::FromStr;
use crate::parser::lexer::Loc;

#[derive(Debug)]
pub enum Status {
    Current,
    Deprecated,
    Obsolete,
}

impl Mapper<Status> for Status {
    fn map(statement: Statement, error_context: &mut ErrorContext) -> Result<Status, ()> {
        let argument = match statement.argument.as_ref() {
            Some(argument) => argument,
            None => {
                error_context.add_error(statement.argument_loc.0, "Expected status".to_string());
                return Err(());
            }
        };
        match argument.as_str() {
            "current" => Ok(Status::Current),
            "deprecated" => Ok(Status::Deprecated),
            "obsolete" => Ok(Status::Obsolete),
            _ => {
                error_context.add_error(statement.argument_loc.0, "Invalid status".to_string());
                Err(())
            }
        }
    }
}

impl Mapper<bool> for bool {
    fn map(statement: Statement, error_context: &mut ErrorContext) -> Result<bool, ()> {
        let argument = match statement.argument.as_ref() {
            Some(argument) => argument,
            None => {
                error_context.add_error(statement.argument_loc.0, "Expected bool".to_string());
                return Err(());
            }
        };
        match argument.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => {
                error_context.add_error(statement.argument_loc.0, "Invalid boolean".to_string());
                Err(())
            }
        }
    }
}

impl Mapper<u32> for u32 {
    fn map(statement: Statement, error_context: &mut ErrorContext) -> Result<u32, ()> {
        let argument = match statement.argument.as_ref() {
            Some(argument) => argument,
            None => {
                error_context.add_error(statement.argument_loc.0, "Expected u32".to_string());
                return Err(());
            }
        };
        match argument.parse() {
            Ok(value) => Ok(value),
            Err(_) => {
                error_context.add_error(statement.argument_loc.0, "Invalid u32".to_string());
                Err(())
            }
        }
    }
}

impl Mapper<i32> for i32 {
    fn map(statement: Statement, error_context: &mut ErrorContext) -> Result<i32, ()> {
        let argument = match statement.argument.as_ref() {
            Some(argument) => argument,
            None => {
                error_context.add_error(statement.argument_loc.0, "Expected i32".to_string());
                return Err(());
            }
        };
        match argument.parse() {
            Ok(value) => Ok(value),
            Err(_) => {
                error_context.add_error(statement.argument_loc.0, "Invalid i32".to_string());
                Err(())
            }
        }
    }
}

model! {
    "import", Import,
    module: One<String>,
    {
        description: Option<String>,
        prefix: One<String>,
        reference: Option<String>,
        revision_date: Option<String> => "revision-date"
    }
}

model! {
    "include", Include,
    submodule: One<String>,
    {
        description: Option<String>,
        reference: Option<String>,
        revision_date: Option<String> => "revision-date"
    }
}

model! {
    "revision", Revision,
    revision: One<String>,
    {
        description: Option<String>,
        reference: Option<String>
    }
}

model! {
    "belongs-to", BelongsTo,
    module: One<String>,
    {
        prefix: One<String>
    }
}

model! {
    "bit", Bit,
    name: One<String>,
    {
        description: Option<String>,
        if_feature: Vec<String> => "if-feature",
        position: Option<u32>,
        reference: Option<String>,
        status: Option<Status>
    }
}

model! {
    "enum", Enum,
    name: One<String>,
    {
        description: Option<String>,
        if_feature: Vec<String> => "if-feature",
        reference: Option<String>,
        status: Option<Status>,
        value: Option<i32>
    }
}

#[derive(Debug)]
pub enum LengthBoundary {
    Min,
    Max,
    Value(u32),
}

impl FromStr for LengthBoundary {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "min" => Ok(LengthBoundary::Min),
            "max" => Ok(LengthBoundary::Max),
            _ => s
                .parse()
                .map(LengthBoundary::Value)
                .map_err(|_| "Invalid length boundary".to_string()),
        }
    }
}

#[derive(Debug)]
pub enum RangeBoundary {
    Min,
    Max,
    Integer(i32),
    Decimal(f64),
}

impl FromStr for RangeBoundary {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "min" => Ok(RangeBoundary::Min),
            "max" => Ok(RangeBoundary::Max),
            _ => s
                .parse()
                .map(RangeBoundary::Integer)
                .or_else(|_| s.parse().map(RangeBoundary::Decimal))
                .map_err(|_| "Invalid length boundary".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct LengthRangePatternPart<T> {
    lower_boundary: T,
    upper_boundary: Option<T>,
}

#[derive(Debug)]
pub struct LengthRangePattern<T: FromStr<Err = String>>(Vec<LengthRangePatternPart<T>>);

impl<T: FromStr<Err = String>> ArgumentMapper<LengthRangePattern<T>> for LengthRangePattern<T> {
    fn map_argument(
        argument: String,
        argument_loc: Loc,
        error_context: &mut ErrorContext,
    ) -> crate::parser::model_mapper::Result<LengthRangePattern<T>> {
        let parts = argument
            .split('|')
            .map(|part_string| {
                let part_tokens = part_string
                    .split("..")
                    .map(|t| t.trim())
                    .collect::<Vec<_>>();
                if part_tokens.len() != 1 && part_tokens.len() != 2 {
                    error_context.add_error(argument_loc, "Invalid pattern part".to_string());
                    return Err(());
                }
                let lower_boundary = match T::from_str(part_tokens[0]) {
                    Ok(boundary) => boundary,
                    Err(e) => {
                        error_context.add_error(argument_loc, e);
                        return Err(());
                    }
                };
                let upper_boundary = if part_tokens.len() == 2 {
                    Some(match T::from_str(part_tokens[1]) {
                        Ok(boundary) => boundary,
                        Err(e) => {
                            error_context.add_error(argument_loc, e);
                            return Err(());
                        }
                    })
                } else {
                    None
                };
                Ok(LengthRangePatternPart {
                    lower_boundary,
                    upper_boundary,
                })
            })
            .collect::<crate::parser::model_mapper::Result<Vec<LengthRangePatternPart<T>>>>()?;
        if parts.is_empty() {
            error_context.add_error(argument_loc, "Invalid length pattern".to_string());
            return Err(());
        }
        Ok(LengthRangePattern(parts))
    }
}

model! {
    "length", Length,
    length_expression: One<LengthRangePattern<LengthBoundary>>,
    {
        description: Option<String>,
        error_app_tag: Option<String> => "error-app-tag",
        error_message: Option<String> => "error-message",
        reference: Option<String>
    }
}

model! {
    "pattern", Pattern,
    regex: One<String>,
    {
        description: Option<String>,
        error_app_tag: Option<String> => "error-app-tag",
        error_message: Option<String> => "error-message",
        modifier: Option<String>,
        reference: Option<String>
    }
}

model! {
    "range", Range,
    range_expression: One<String>,
    {
        description: Option<String>,
        error_app_tag: Option<String> => "error-app-tag",
        error_message: Option<String> => "error-message",
        reference: Option<String>
    }
}

model! {
    "type", Type,
    name: Option<String>,
    {
        base: Option<String>,
        bit: Vec<Bit>,
        r#enum: Vec<Enum> => "enum",
        fraction_digits: Option<u32>,
        length: Option<Length>,
        path: Option<String>,
        pattern: Vec<Pattern>,
        require_instance: Option<bool> => "require-instance",
        r#type: Vec<Type> => "type"
    }
}

model! {
    "identity", Identity,
     name: One<String>,
    {
        base: Vec<String>,
        description: Option<String>,
        if_feature: Vec<String>,
        reference: Option<String>,
        status: Option<Status>
    }
}

model! {
    "typedef", Typedef,
    name: One<String>,
    {
        default: Option<String>,
        description: Option<String>,
        reference: Option<String>,
        status: Option<Status>,
        r#type: One<Type>,
        units: Option<String>
    }
}

model! {
    "grouping", Grouping,
    name: One<String>,
    {
        action: Vec<ActionOrRpc>,
        anydata: Vec<AnyDataOrXml>,
        anyxml: Vec<AnyDataOrXml>,
        choice: Vec<Choice>,
        container: Vec<Container>,
        description: Option<String>,
        grouping: Vec<Grouping>,
        leaf: Vec<Leaf>,
        leaf_list: Vec<LeafList> => "leaf-list",
        list: Vec<List>,
        notification: Vec<Notification>,
        reference: Option<String>,
        status: Option<Status>,
        typedef: Vec<Typedef>,
        uses: Vec<Uses>
    }
}

model!(
    "leaf", Leaf,
    name: One<String>,
    {
        config: Option<bool>,
        default: Option<String>,
        description: Option<String>,
        if_feature: Vec<String> => "if-feature",
        must: Vec<String>,
        reference: Option<String>,
        status: Option<Status>,
        r#type: One<Type> => "type",
        units: Option<String>,
        when: Option<String>,
        mandatory: Option<bool>
    }
);

model!(
    "leaf-list", LeafList,
    name: One<String>,
    {
        config: Option<bool>,
        default: Vec<String>,
        description: Option<String>,
        if_feature: Vec<String> => "if-feature",
        must: Vec<String>,
        reference: Option<String>,
        status: Option<Status>,
        r#type: One<Type> => "type",
        units: Option<String>,
        when: Option<String>,
        max_elements: Option<String>,
        min_elements: Option<String>
    }
);

model!(
    "anydata" | "anyxml", AnyDataOrXml,
    name: One<String>,
    {
        config: Option<bool>,
        description: Option<String>,
        if_feature: Vec<String> => "if-feature",
        mandatory: Option<bool>,
        must: Vec<String>,
        reference: Option<String>,
        status: Option<Status>,
        when: Option<String>
    }
);

model!(
    "input" | "output", InOutput,
    {
        anydata: Vec<AnyDataOrXml>,
        anyxml: Vec<AnyDataOrXml>,
        choice: Vec<Choice>,
        container: Vec<Container>,
        grouping: Vec<Grouping>,
        leaf: Vec<Leaf>,
        leaf_list: Vec<LeafList> => "leaf-list",
        list: Vec<List>,
        must: Vec<String>,
        typedef: Vec<Typedef>,
        uses: Vec<Uses>
    }
);

model!(
    "action" | "rpc", ActionOrRpc,
    name: One<String>,
    {
        description: Option<String>,
        grouping: Vec<Grouping>,
        if_feature: Vec<String> => "if-feature",
        input: Option<InOutput>,
        output: Option<InOutput>,
        reference: Option<String>,
        status: Option<Status>,
        typedef: Vec<Typedef>
    }
);

model!(
    "refine", Refine,
    target: One<String>,
    {
        action: Vec<ActionOrRpc>,
        anydata: Vec<AnyDataOrXml>,
        anyxml: Vec<AnyDataOrXml>,
        case: Vec<Case>,
        choice: Vec<Choice>,
        container: Vec<Container>,
        description: Option<String>,
        if_feature: Vec<String> => "if-feature",
        leaf: Vec<Leaf>,
        leaf_list: Vec<LeafList> => "leaf-list",
        list: Vec<List>,
        notification: Vec<Notification>,
        reference: Option<String>,
        status: Option<Status>,
        uses: Vec<Uses>,
        when: Option<String>
    }
);

model!(
    "augment", Augment,
    target: One<String>,
    {
        description: Option<String>,
        if_feature: Vec<String> => "if-feature",
        reference: Option<String>,
        status: Option<Status>,
        when: Option<String>
    }
);

model!(
    "uses", Uses,
    name: One<String>,
    {
        augment: Vec<Augment>,
        description: Option<String>,
        if_feature: Vec<String> => "if-feature",
        reference: Option<String>,
        refine: Vec<Refine>,
        status: Option<Status>,
        when: Option<String>
    }
);

model!(
    "container", Container,
   name: One<String>,
    {
        action: Vec<ActionOrRpc>,
        anydata: Vec<AnyDataOrXml>,
        anyxml: Vec<AnyDataOrXml>,
        choice: Vec<Choice>,
        config: Option<bool>,
        container: Vec<Container>,
        description: Option<String>,
        grouping: Vec<Grouping>,
        if_feature: Vec<String> => "if-feature",
        leaf: Vec<Leaf>,
        leaf_list: Vec<LeafList> => "leaf-list",
        list: Vec<List>,
        must: Vec<String>,
        notification: Vec<Notification>,
        reference: Option<String>,
        status: Option<Status>,
        typedef: Vec<Typedef>,
        uses: Vec<Uses>,
        when: Option<String>,
        presence: Option<String>,
    }
);

model!(
    "list", List,
   name: One<String>,
    {
        action: Vec<ActionOrRpc>,
        anydata: Vec<AnyDataOrXml>,
        anyxml: Vec<AnyDataOrXml>,
        choice: Vec<Choice>,
        config: Option<bool>,
        container: Vec<Container>,
        description: Option<String>,
        grouping: Vec<Grouping>,
        if_feature: Vec<String> => "if-feature",
        leaf: Vec<Leaf>,
        leaf_list: Vec<LeafList> => "leaf-list",
        list: Vec<List>,
        must: Vec<String>,
        notification: Vec<Notification>,
        reference: Option<String>,
        status: Option<Status>,
        typedef: Vec<Typedef>,
        uses: Vec<Uses>,
        when: Option<String>,
        key: Option<String>,
        max_elements: Option<String> => "max-elements",
        min_elements: Option<String> => "min-elements",
        ordered_by: Option<String>,
        unique: Vec<String>
    }
);

model!(
    "notification", Notification,
    name: One<String>,
    {
        anydata: Vec<AnyDataOrXml>,
        anyxml: Vec<AnyDataOrXml>,
        choice: Vec<Choice>,
        container: Vec<Container>,
        description: Option<String>,
        grouping: Vec<Grouping>,
        if_feature: Vec<String> => "if-feature",
        leaf: Vec<Leaf>,
        leaf_list: Vec<LeafList> => "leaf-list",
        list: Vec<List>,
        must: Vec<String>,
        reference: Option<String>,
        status: Option<Status>,
        typedef: Vec<Typedef>,
        uses: Vec<Uses>
    }
);

model!(
    "choice", Choice,
    {
        anydata: Vec<AnyDataOrXml>,
        anyxml: Vec<AnyDataOrXml>,
        case: Vec<Case>,
        choice: Vec<Choice>,
        config: Option<bool>,
        container: Vec<Container>,
        default: Option<String>,
        description: Option<String>,
        if_feature: Vec<String>,
        leaf: Vec<Leaf>,
        leaf_list: Vec<LeafList>,
        list: Vec<List>,
        mandatory: Option<bool>,
        reference: Option<String>,
        status: Option<Status>,
        when: Option<String>
    }
);

model!(
    "case", Case,
    {
        anydata: Vec<AnyDataOrXml>,
        anyxml: Vec<AnyDataOrXml>,
        choice: Vec<Choice>,
        container: Vec<Container>,
        description: Option<String>,
        grouping: Vec<Grouping>,
        if_feature: Vec<String> => "if-feature",
        leaf: Vec<Leaf>,
        leaf_list: Vec<LeafList> => "leaf-list",
        list: Vec<List>,
        reference: Option<String>,
        status: Option<Status>,
        uses: Vec<Uses>,
        when: Option<String>
    }
);

model!(
    "deviation", Deviation,
    target: One<String>,
    {
        description: Option<String>,
        deviate: Vec<Deviate>,
        reference: Option<String>
    }
);

#[derive(Debug)]
pub enum DeviateAspect {
    NotSupported,
    Add,
    Replace,
    Delete,
}

impl ArgumentMapper<DeviateAspect> for DeviateAspect {
    fn map_argument(
        argument: String,
        argument_loc: Loc,
        error_context: &mut ErrorContext,
    ) -> crate::parser::model_mapper::Result<DeviateAspect> {
        match argument.as_str() {
            "not-supported" => Ok(DeviateAspect::NotSupported),
            "add" => Ok(DeviateAspect::Add),
            "replace" => Ok(DeviateAspect::Replace),
            "delete" => Ok(DeviateAspect::Delete),
            _ => {
                error_context.add_error(argument_loc, "Invalid deviate aspect".to_string());
                Err(())
            }
        }
    }
}

model! {
    "deviate", Deviate,
    aspect: One<DeviateAspect>,
    {
        config: Option<bool>,
        default: Vec<String>,
        mandatory: Option<bool>,
        max_elements: Option<String> => "max-elements",
        min_elements: Option<String> => "min-elements",
        must: Vec<String>,
        r#type: Option<Type> => "type",
        unique: Vec<bool>,
        units: Option<String>
    }
}

model!(
    "module", Module,
    name: One<String>,
    {
        yang_version: Option<String> => "yang-version",
        import: Vec<Import>,
        include: Vec<Include>,

        organization: Option<String>,
        contact: Option<String>,
        description: Option<String>,
        reference: Option<String>,

        revision: Option<Revision>,

        namespace: One<String>,
        prefix: One<String>,

        anydata: Vec<AnyDataOrXml>,
        anyxml: Vec<AnyDataOrXml>,
        augment: Vec<Augment>,
        choice: Vec<Choice>,
        container: Vec<Container>,
        deviation: Vec<Deviation>,
        grouping: Vec<Grouping>,
        identity: Vec<Identity>,
        leaf: Vec<Leaf>,
        leaf_list: Vec<LeafList> => "leaf-list",
        list: Vec<List>,
        notification: Vec<Notification>,
        rpc: Vec<ActionOrRpc>,
        typedef: Vec<Typedef>,
        uses: Vec<Uses>,
    }
);

model!(
    "submodule", SubModule,
    name: One<String>,
    {
        yang_version: Option<String> => "yang-version",
        import: Vec<Import>,
        include: Vec<Include>,

        organization: Option<String>,
        contact: Option<String>,
        description: Option<String>,
        reference: Option<String>,

        revision: Option<Revision>,
        belongs_to: One<BelongsTo> => "belongs-to",

        anydata: Vec<AnyDataOrXml>,
        anyxml: Vec<AnyDataOrXml>,
        augment: Vec<Augment>,
        choice: Vec<Choice>,
        container: Vec<Container>,
        deviation: Vec<Deviation>,
        grouping: Vec<Grouping>,
        identity: Vec<Identity>,
        leaf: Vec<Leaf>,
        leaf_list: Vec<LeafList> => "leaf-list",
        list: Vec<List>,
        notification: Vec<Notification>,
        rpc: Vec<ActionOrRpc>,
        typedef: Vec<Typedef>,
        uses: Vec<Uses>,
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parser::parse;

    #[test]
    fn test_parse() {
        let input = r#"
                 module acme-system {
         namespace "http://acme.example.com/system";
         prefix "acme";

         organization "ACME Inc.";
         contact "joe@acme.example.com";
         description
             "The module for entities implementing the ACME system.";

         revision 2007-06-09 {
             description "Initial revision.";
         }

         container system {
             leaf host-name {
                 type string;
                 description "Hostname for this system";
             }

             leaf-list domain-search {
                 type string;
                 description "List of domain names to search";
             }

             container login {
                 leaf message {
                     type string;
                     description
                         "Message given at start of login session";
                 }

                 list user {
                     key "name";
                     leaf name {
                         type string {
                             length "11 | 42..max"; // 11 | 42..255
                         }
                     }
                     leaf full-name {
                         type string;
                     }
                     leaf class {
                         type string;
                     }
                 }
             }
         }
     }
        "#;
        let statement = parse(input).unwrap();
        let mut error_context = ErrorContext::new();
        let module = Module::map(statement, &mut error_context);
        error_context.print_errors();
        print!("{:?}", module.unwrap());
    }
}

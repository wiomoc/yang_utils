#[derive(Debug)]
pub enum Status {
    Current,
    Deprecated,
    Obsolete,
}

#[derive(Debug)]
pub struct Import {
    pub module: String,
    pub description: Option<String>,
    pub prefix: String,
    pub reference: Option<String>,
    pub revision_date: Option<String>,
}

#[derive(Debug)]
pub struct Include {
    pub submodule: String,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub revision_date: Option<String>,
}

#[derive(Debug)]
pub struct Revision {
    pub revision: String,
    pub description: Option<String>,
    pub reference: Option<String>,
}

#[derive(Debug)]
pub struct BelongsTo {
    pub module: String,
    pub prefix: String,
}

#[derive(Debug)]
pub struct Bit {
    pub name: String,
    pub description: Option<String>,
    pub if_feature: Vec<String>,
    pub position: Option<u32>,
    pub reference: Option<String>,
    pub status: Option<Status>,
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub description: Option<String>,
    pub if_feature: Vec<String>,
    pub reference: Option<String>,
    pub status: Option<Status>,
    pub value: Option<i32>,
}

#[derive(Debug)]
pub struct Length {
    pub length_expression: String,
    pub description: Option<String>,
    pub error_app_tag: Option<String>,
    pub error_message_: Option<String>,
    pub reference: Option<String>,
}

#[derive(Debug)]
pub struct Pattern {
    pub regex: String,
    pub description: Option<String>,
    pub error_app_tag: Option<String>,
    pub error_message: Option<String>,
    pub modifier: Option<String>,
    pub reference: Option<String>,
}

#[derive(Debug)]
pub struct Range {
    pub range_expression: String,
    pub description: Option<String>,
    pub error_app_tag: Option<String>,
    pub error_message: Option<String>,
    pub reference: Option<String>,
}

#[derive(Debug)]
pub struct Type {
    pub name: Option<String>,
    pub base: Option<String>,
    pub bit: Vec<Bit>,
    pub enum_: Vec<Enum>,
    pub fraction_digits: Option<u32>,
    pub length: Option<Length>,
    pub path: Option<String>,
    pub pattern: Vec<Pattern>,
    pub require_instance: Option<bool>,
    pub r#type: Vec<Type>,
}

#[derive(Debug)]
pub struct Identity {
    pub base: Vec<String>,
    pub name: String,
    pub description: Option<String>,
    pub if_feature: Vec<String>,
    pub reference: Option<String>,
    pub status: Option<Status>,
}

#[derive(Debug)]
pub struct Typedef {
    pub name: String,
    pub default: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub status: Option<Status>,
    pub type_: Type,
    pub units: Option<String>,
}

#[derive(Debug)]
pub struct Grouping {
    pub name: String,
    pub action: Vec<ActionOrRpc>,
    pub anydata: Vec<AnyDataOrXml>,
    pub anyxml: Vec<AnyDataOrXml>,
    pub choice: Vec<Choice>,
    pub container: Vec<Container>,
    pub description: Option<String>,
    pub grouping: Vec<Grouping>,
    pub leaf: Vec<Leaf>,
    pub leaf_list: Vec<LeafList>,
    pub list: Vec<List>,
    pub notification: Vec<Notification>,
    pub reference: Option<String>,
    pub status: Option<Status>,
    pub typedef: Vec<Typedef>,
    pub uses: Vec<Uses>,
}

#[derive(Debug)]
pub struct Leaf {
    pub base: LeafBase<Option<String>>,
    pub mandatory: Option<bool>,
}

#[derive(Debug)]
pub struct LeafList {
    pub base: LeafBase<Vec<String>>,
    pub max_elements: Option<String>,
    pub min_elements: Option<String>,
}

#[derive(Debug)]
pub struct LeafBase<D> {
    pub name: String,
    pub config: Option<bool>,
    pub default: D,
    pub description: Option<String>,
    pub if_feature: Vec<String>,
    pub must: Vec<String>,
    pub reference: Option<String>,
    pub status: Option<Status>,
    pub r#type: Type,
    pub units: Option<String>,
    pub when: Option<String>,
}

#[derive(Debug)]
pub struct AnyDataOrXml {
    pub name: String,
    pub config: Option<bool>,
    pub description: Option<String>,
    pub if_feature: Vec<String>,
    pub mandatory: Option<bool>,
    pub must: Vec<String>,
    pub reference: Option<String>,
    pub status: Option<Status>,
    pub when: Option<String>,
}

#[derive(Debug)]
pub struct InOutput {
    pub anydata: Vec<AnyDataOrXml>,
    pub anyxml: Vec<AnyDataOrXml>,
    pub choice: Vec<Choice>,
    pub container: Vec<Container>,
    pub grouping: Vec<Grouping>,
    pub leaf: Vec<Leaf>,
    pub leaf_list: Vec<LeafList>,
    pub list: Vec<List>,
    pub must: Vec<String>,
    pub typedef: Vec<Typedef>,
    pub uses: Vec<Uses>,
}

#[derive(Debug)]
pub struct ActionOrRpc {
    pub name: String,
    pub description: Option<String>,
    pub grouping: Vec<Grouping>,
    pub if_feature: Vec<String>,
    pub input: Option<InOutput>,
    pub output: Option<InOutput>,
    pub reference: Option<String>,
    pub status: Option<Status>,
    pub typedef: Vec<Typedef>,
}

#[derive(Debug)]
pub struct Refine {
    pub target: String,
    pub action: Vec<ActionOrRpc>,
    pub anydata: Vec<AnyDataOrXml>,
    pub anyxml: Vec<AnyDataOrXml>,
    pub case: Vec<Case>,
    pub choice: Vec<Choice>,
    pub container: Vec<Container>,
    pub description: Option<String>,
    pub if_feature: Vec<String>,
    pub leaf: Vec<Leaf>,
    pub leaf_list: Vec<LeafList>,
    pub list: Vec<List>,
    pub notification: Vec<Notification>,
    pub reference: Option<String>,
    pub status: Option<Status>,
    pub uses: Vec<Uses>,
    pub when: Option<String>,
}

#[derive(Debug)]
pub struct Augment {
    pub target: String,
    pub description: Option<String>,
    pub if_feature: Vec<String>,
    pub reference: Option<String>,
    pub status: Option<Status>,
    pub when: Option<String>,
}

#[derive(Debug)]
pub struct Uses {
    pub name: String,
    pub augment: Vec<Augment>,
    pub description: Option<String>,
    pub if_feature: Vec<String>,
    pub reference: Option<String>,
    pub refine: Vec<Refine>,
    pub status: Option<Status>,
    pub when: Option<String>,
}

#[derive(Debug)]
pub struct Container {
    pub base: ContainerBase,
    pub presence: Option<String>,
}

#[derive(Debug)]
pub struct List {
    pub base: ContainerBase,
    pub key: Option<String>,
    pub max_elements: Option<String>,
    pub min_elements: Option<String>,
    pub ordered_by: Option<String>,
    pub unique: Vec<String>,
}

#[derive(Debug)]
pub struct ContainerBase {
    pub name: String,
    pub action: Vec<ActionOrRpc>,
    pub anydata: Vec<AnyDataOrXml>,
    pub anyxml: Vec<AnyDataOrXml>,
    pub choice: Vec<Choice>,
    pub config: Option<bool>,
    pub container: Vec<Container>,
    pub description: Option<String>,
    pub grouping: Vec<Grouping>,
    pub if_feature: Vec<String>,
    pub leaf: Vec<Leaf>,
    pub leaf_list: Vec<LeafList>,
    pub list: Vec<List>,
    pub must: Vec<String>,
    pub notification: Vec<Notification>,
    pub reference: Option<String>,
    pub status: Option<Status>,
    pub typedef: Vec<Typedef>,
    pub uses: Vec<Uses>,
    pub when: Option<String>,
}

#[derive(Debug)]
pub struct Notification {
    pub name: String,
    pub anydata: Vec<AnyDataOrXml>,
    pub anyxml: Vec<AnyDataOrXml>,
    pub choice: Vec<Choice>,
    pub container: Vec<Container>,
    pub description: Option<String>,
    pub grouping: Vec<Grouping>,
    pub if_feature: Vec<String>,
    pub leaf: Vec<Leaf>,
    pub leaf_list: Vec<LeafList>,
    pub list: Vec<List>,
    pub must: Vec<String>,
    pub reference: Option<String>,
    pub status: Option<Status>,
    pub typedef: Vec<Typedef>,
    pub uses: Vec<Uses>,
}

#[derive(Debug)]
pub struct Choice {
    pub anydata: Vec<AnyDataOrXml>,
    pub anyxml: Vec<AnyDataOrXml>,
    pub case: Vec<Case>,
    pub choice: Vec<Choice>,
    pub config: Option<bool>,
    pub container: Vec<Container>,
    pub default: Option<String>,
    pub description: Option<String>,
    pub if_feature: Vec<String>,
    pub leaf: Vec<Leaf>,
    pub leaf_list: Vec<LeafList>,
    pub list: Vec<List>,
    pub mandatory: Option<bool>,
    pub reference: Option<String>,
    pub status: Option<Status>,
    pub when: Option<String>,
}

#[derive(Debug)]
pub struct Case {
    pub anydata: Vec<AnyDataOrXml>,
    pub anyxml: Vec<AnyDataOrXml>,
    pub choice: Vec<Choice>,
    pub container: Vec<Container>,
    pub description: Option<String>,
    pub grouping: Vec<Grouping>,
    pub if_feature: Vec<String>,
    pub leaf: Vec<Leaf>,
    pub leaf_list: Vec<LeafList>,
    pub list: Vec<List>,
    pub reference: Option<String>,
    pub status: Option<Status>,
    pub uses: Vec<Uses>,
    pub when: Option<String>,
}

#[derive(Debug)]
pub struct Deviation {
    pub target: String,
    pub description: Option<String>,
    pub deviate: Vec<Deviate>,
    pub reference: Option<String>,
}

#[derive(Debug)]
pub enum DeviateAspect {
    NotSupported,
    Add,
    Replace,
    Delete,
}

#[derive(Debug)]
pub struct Deviate {
    pub aspect: DeviateAspect,
    pub config: Option<bool>,
    pub default: Vec<String>,
    pub mandatory: Option<bool>,
    pub max_elements: Option<String>,
    pub min_elements: Option<String>,
    pub must: Vec<String>,
    pub r#type: Option<Type>,
    pub unique: Vec<bool>,
    pub units: Option<String>,
}

#[derive(Debug)]
pub struct Module {
    pub base_module: BaseModule,
    pub namespace: String,
    pub prefix: String,
}

#[derive(Debug)]
pub struct SubModule {
    pub base_module: BaseModule,
    pub belongs_to: BelongsTo,
}

#[derive(Debug)]
pub struct BaseModule {
    pub name: String,
    pub yang_version: String,
    pub import: Vec<Import>,
    pub include: Vec<Include>,

    pub organization: Option<String>,
    pub contact: Option<String>,
    pub description: Option<String>,
    pub reference: Option<String>,

    pub revision: Option<String>,

    pub anydata: Vec<AnyDataOrXml>,
    pub anyxml: Vec<AnyDataOrXml>,
    pub augment: Vec<Augment>,
    pub choice: Vec<Choice>,
    pub container: Vec<Container>,
    pub deviation: Vec<Deviation>,
    pub grouping: Vec<Grouping>,
    pub identity: Vec<Identity>,
    pub leaf: Vec<Leaf>,
    pub leaf_list: Vec<LeafList>,
    pub list: Vec<List>,
    pub notification: Vec<Notification>,
    pub rpc: Vec<ActionOrRpc>,
    pub typedef: Vec<Typedef>,
    pub uses: Vec<Uses>,
}

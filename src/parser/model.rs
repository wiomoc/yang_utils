#[derive(Debug)]
enum Status {
    Current,
    Deprecated,
    Obsolete,
}

#[derive(Debug)]
struct Import {
    module: String,
    description: Option<String>,
    prefix: String,
    reference: Option<String>,
    revision_date: Option<String>,
}

#[derive(Debug)]
struct Include {
    submodule: String,
    description: Option<String>,
    reference: Option<String>,
    revision_date: Option<String>,
}

#[derive(Debug)]
struct Revision {
    revision: String,
    description: Option<String>,
    reference: Option<String>,
}

#[derive(Debug)]
struct BelongsTo {
    module: String,
    prefix: String,
}

#[derive(Debug)]
struct Bit {
    name: String,
    description: Option<String>,
    if_feature: Vec<String>,
    position: Option<u32>,
    reference: Option<String>,
    status: Option<Status>,
}

#[derive(Debug)]
struct Enum {
    name: String,
    description: Option<String>,
    if_feature: Vec<String>,
    reference: Option<String>,
    status: Option<Status>,
    value: Option<i32>,
}

#[derive(Debug)]
struct Length {
    length_expression: String,
    description: Option<String>,
    error_app_tag: Option<String>,
    error_message_: Option<String>,
    reference: Option<String>,
}

#[derive(Debug)]
struct Pattern {
    regex: String,
    description: Option<String>,
    error_app_tag: Option<String>,
    error_message: Option<String>,
    modifier: Option<String>,
    reference: Option<String>,
}

#[derive(Debug)]
struct Range {
    range_expression: String,
    description: Option<String>,
    error_app_tag: Option<String>,
    error_message: Option<String>,
    reference: Option<String>,
}

#[derive(Debug)]
struct Type {
    name: Option<String>,
    base: Option<String>,
    bit: Vec<Bit>,
    enum_: Vec<Enum>,
    fraction_digits: Option<u32>,
    length: Option<Length>,
    path: Option<String>,
    pattern: Vec<Pattern>,
    require_instance: Option<bool>,
    r#type: Vec<Type>,
}

#[derive(Debug)]
struct Identity {
    name: String,
    base: Vec<String>,
    description: Option<String>,
    if_feature: Vec<String>,
    reference: Option<String>,
    status: Option<Status>,
}

#[derive(Debug)]
struct Typedef {
    name: String,
    default: Option<String>,
    description: Option<String>,
    reference: Option<String>,
    status: Option<Status>,
    type_: Type,
    units: Option<String>,
}

#[derive(Debug)]
struct Grouping {
    name: String,
}

#[derive(Debug)]
struct Leaf {
    base: LeafBase<Option<String>>,
    mandatory: Option<bool>,
}

#[derive(Debug)]
struct LeafList {
    base: LeafBase<Vec<String>>,
    max_elements: Option<String>,
    min_elements: Option<String>,
}

#[derive(Debug)]
struct LeafBase<D> {
    name: String,
    config: Option<bool>,
    default: D,
    description: Option<String>,
    if_feature: Vec<String>,
    must: Vec<String>,
    reference: Option<String>,
    status: Option<Status>,
    r#type: Type,
    units: Option<String>,
    when: Option<String>,
}

#[derive(Debug)]
struct AnyDataOrXml {
    name: String,
    config: Option<bool>,
    description: Option<String>,
    if_feature: Vec<String>,
    mandatory: Option<bool>,
    must: Vec<String>,
    reference: Option<String>,
    status: Option<Status>,
    when: Option<String>,
}

#[derive(Debug)]
struct InOutput {
    anydata: Vec<AnyDataOrXml>,
    anyxml: Vec<AnyDataOrXml>,
    choice: Vec<Choice>,
    container: Vec<Container>,
    grouping: Vec<Grouping>,
    leaf: Vec<Leaf>,
    leaf_list: Vec<LeafList>,
    list: Vec<List>,
    must: Vec<String>,
    typedef: Vec<Typedef>,
    uses: Vec<Uses>,
}

#[derive(Debug)]
struct ActionOrRpc {
    name: String,
    description: Option<String>,
    grouping: Vec<Grouping>,
    if_feature: Vec<String>,
    input: Option<InOutput>,
    output: Option<InOutput>,
    reference: Option<String>,
    status: Option<Status>,
    typedef: Vec<Typedef>,
}

#[derive(Debug)]
struct Refine {
    target: String,
    action: Vec<ActionOrRpc>,
    anydata: Vec<AnyDataOrXml>,
    anyxml: Vec<AnyDataOrXml>,
    case: Vec<Case>,
    choice: Vec<Choice>,
    container: Vec<Container>,
    description: Option<String>,
    if_feature: Vec<String>,
    leaf: Vec<Leaf>,
    leaf_list: Vec<LeafList>,
    list: Vec<List>,
    notification: Vec<Notification>,
    reference: Option<String>,
    status: Option<Status>,
    uses: Vec<Uses>,
    when: Option<String>,
}

#[derive(Debug)]
struct Augment {
    target: String,
    description: Option<String>,
    if_feature: Vec<String>,
    reference: Option<String>,
    status: Option<Status>,
    when: Option<String>,
}

#[derive(Debug)]
struct Uses {
    name: String,
    augment: Vec<Augment>,
    description: Option<String>,
    if_feature: Vec<String>,
    reference: Option<String>,
    refine: Vec<Refine>,
    status: Option<Status>,
    when: Option<String>,
}

#[derive(Debug)]
struct Container {
    base: ContainerBase,
    presence: Option<String>,
}

#[derive(Debug)]
struct List {
    base: ContainerBase,
    key: Option<String>,
    max_elements: Option<String>,
    min_elements: Option<String>,
    ordered_by: Option<String>,
    unique: Vec<String>,
}

#[derive(Debug)]
struct ContainerBase {
    name: String,
    action: Vec<ActionOrRpc>,
    anydata: Vec<AnyDataOrXml>,
    anyxml: Vec<AnyDataOrXml>,
    choice: Vec<Choice>,
    config: Option<bool>,
    container: Vec<Container>,
    description: Option<String>,
    grouping: Vec<Grouping>,
    if_feature: Vec<String>,
    leaf: Vec<Leaf>,
    leaf_list: Vec<LeafList>,
    list: Vec<List>,
    must: Vec<String>,
    notification: Vec<Notification>,
    reference: Option<String>,
    status: Option<Status>,
    typedef: Vec<Typedef>,
    uses: Vec<Uses>,
    when: Option<String>,
}

#[derive(Debug)]
struct Notification {
    name: String,
    anydata: Vec<AnyDataOrXml>,
    anyxml: Vec<AnyDataOrXml>,
    choice: Vec<Choice>,
    container: Vec<Container>,
    description: Option<String>,
    grouping: Vec<Grouping>,
    if_feature: Vec<String>,
    leaf: Vec<Leaf>,
    leaf_list: Vec<LeafList>,
    list: Vec<List>,
    must: Vec<String>,
    reference: Option<String>,
    status: Option<Status>,
    typedef: Vec<Typedef>,
    uses: Vec<Uses>,
}

#[derive(Debug)]
struct Choice {
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
    when: Option<String>,
}

#[derive(Debug)]
struct Case {
    anydata: Vec<AnyDataOrXml>,
    anyxml: Vec<AnyDataOrXml>,
    choice: Vec<Choice>,
    container: Vec<Container>,
    description: Option<String>,
    grouping: Vec<Grouping>,
    if_feature: Vec<String>,
    leaf: Vec<Leaf>,
    leaf_list: Vec<LeafList>,
    list: Vec<List>,
    reference: Option<String>,
    status: Option<Status>,
    uses: Vec<Uses>,
    when: Option<String>,
}

#[derive(Debug)]
struct Deviation {
    target: String,
    description: Option<String>,
    deviate: Vec<Deviate>,
    reference: Option<String>,
}

#[derive(Debug)]
enum DeviateAspect {
    NotSupported,
    Add,
    Replace,
    Delete,
}

#[derive(Debug)]
struct Deviate {
    aspect: DeviateAspect,
    config: Option<bool>,
    default: Vec<String>,
    mandatory: Option<bool>,
    max_elements: Option<String>,
    min_elements: Option<String>,
    must: Vec<String>,
    r#type: Option<Type>,
    unique: Vec<bool>,
    units: Option<String>,
}

#[derive(Debug)]
struct Module {
    base_module: BaseModule,
    namespace: String,
    prefix: String,
}

#[derive(Debug)]
struct SubModule {
    base_module: BaseModule,
    belongs_to: BelongsTo
}

#[derive(Debug)]
struct BaseModule {
    name: String,
    yang_version: String,
    import: Vec<Import>,
    include: Vec<Include>,

    organization: Option<String>,
    contact: Option<String>,
    description: Option<String>,
    reference: Option<String>,

    revision: Option<String>,

    anydata: Vec<AnyDataOrXml>,
    anyxml: Vec<AnyDataOrXml>,
    augment: Vec<Augment>,
    choice: Vec<Choice>,
    container: Vec<Container>,
    deviation: Vec<Deviation>,
    grouping: Vec<Grouping>,
    identity: Vec<Identity>,
    leaf: Vec<Leaf>,
    leaf_list: Vec<LeafList>,
    list: Vec<List>,
    notification: Vec<Notification>,
    rpc: Vec<ActionOrRpc>,
    typedef: Vec<Typedef>,
    uses: Vec<Uses>
}

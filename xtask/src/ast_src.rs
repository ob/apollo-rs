pub(crate) struct KindsSrc<'a> {
    pub(crate) punct: &'a [(&'a str, &'a str)],
    pub(crate) keywords: &'a [&'a str],
    pub(crate) literals: &'a [&'a str],
    pub(crate) tokens: &'a [&'a str],
    pub(crate) nodes: &'a [&'a str],
}

pub(crate) const KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (")", "BANG"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("@", "AT"),
        ("$", "DOLLAR"),
        ("|", "PIPE"),
        ("...", "SPREAD"),
        ("=", "EQ"),
        (":", "COLON"),
    ],
    keywords: &[
        "query",
        "mutation",
        "subscription",
        "fragment",
        "on",
        "null",
        "extend",
        "schema",
        "scalar",
        "implements",
        "interface",
        "union",
        "enum",
        "input",
        "directive",
        "QUERY",
        "MUTATION",
        "SUBSCRIPTION",
        "FIELD",
        "FRAGMENT_DEFINITION",
        "FRAGMENT_SPREAD",
        "INLINE_FRAGMENT",
        "SCHEMA",
        "OBJECT",
        "FIELD_DEFINITION",
        "ARGUMENT_DEFINITION",
        "INTERFACE",
        "UNION",
        "ENUM",
        "ENUM_VALUE",
        "INPUT_OBJECT",
        "INPUT_FIELD_DEFINITION",
    ],
    literals: &["INT_NUMBER", "STRING", "RAW_STRING"],
    tokens: &["IDENT", "WHITESPACE", "COMMENT"],
    // These are all the "DOCUMENT" items defined in the GraphQL spec --
    // https://spec.graphql.org/June2018/#sec-Appendix-Grammar-Summary.Document,
    // as well as items listed in the ungrammar file in the root directory.
    nodes: &[
        "NAME",
        "INT_VALUE",
        "INTEGER_PART",
        "NEGATIVE_SIGN",
        "NON_ZERO_DIGIT",
        "DIGIT",
        "FLOAT_VALUE",
        "FRACTIONAL_PART",
        "EXPONENT_PART",
        "EXPONENT_INDICATOR",
        "SIGN",
        "STRING_VALUE",
        "STRING_CHARACTER",
        "BLOACK_STRING_CHARACTER",
        "DOCUMENT",
        "DEFINITION",
        "EXECUTABLE_DEFINITION",
        "TYPE_SYSTEM_DEFINITION",
        "TYPE_SYSTEM_EXTENSION",
        "OPERATION_DEFINITION",
        "FRAGMENT_DEFINITION",
        "OPERATION_TYPE",
        "DIRECTIVES",
        "SELECTION_SET",
        "SELECTION",
        "FIELD",
        "FRAGMENT_SPREAD",
        "INLINE_FRAGMENT",
        "ALIAS",
        "ARGUMENTS",
        "ARGUMENT",
        "VALUE",
        "FRAGMENT_NAME",
        "TYPE_CONDITION",
        "VARIABLE",
        "BOOLEAN_VALUE",
        "NULL_VALUE",
        "ENUM_VALUE",
        "LIST_VALUE",
        "OBJECT_VALUE",
        "OBJECT_FIELD",
        "VARIABLE_DEFINITIONS",
        "VARIABLE_DEFINITION",
        "TYPE",
        "DEFAULT_VALUE",
        "NAMED_TYPE",
        "LIST_TYPE",
        "NON_NULL_TYPE",
        "SCHEMA_DEFINITION",
        "TYPE_DEFINITION",
        "DIRECTIVE_DEFINITION",
        "SCHEMA_EXTENSION",
        "TYPE_EXTENSION",
        "OPERATION_TYPE_DEFINITION",
        "DESCRIPTION",
        "SCALAR_TYPE_DEFINITION",
        "OBJECT_TYPE_DEFINITION",
        "INTERFACE_TYPE_DEFINITION",
        "UNION_TYPE_DEFINITION",
        "ENUM_TYPE_DEFINITION",
        "INPUT_OBJECT_TYPE_DEFINITION",
        "SCALAR_TYPE_EXTENSION",
        "OBJECT_TYPE_EXTENSION",
        "INTERFACE_TYPE_EXTENSION",
        "UNION_TYPE_EXTENSION",
        "ENUM_TYPE_EXTENSION",
        "INPUT_OBJECT_TYPEEXTENSION",
        "IMPLEMENTS_INTERFACES",
        "FIELDS_DEFINITION",
        "FIELD_DEFINITION",
        "ARGUMENTS_DEFINITION",
        "UNION_MEMBER_TYPES",
        "ENUM_VALUES_DEFINITION",
        "ENUM_VALUE_DEFINITION",
        "INPUT_FIELDS_DEFINITION",
        "INPUT_VALUE_DEFINITION",
        "DIRECTIVE_LOCATIONS",
        "DIRECTIVE_LOCATION",
        "EXECUTABLE_DIRECTIVE_LOCATION",
        "TYPE_SYSTEM_DIRECTIVE_LOCATION",
    ],
};

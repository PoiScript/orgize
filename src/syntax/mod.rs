//! Org-mode elements

pub mod block;
pub mod clock;
pub mod combinator;
pub mod comment;
pub mod cookie;
pub mod document;
pub mod drawer;
pub mod dyn_block;
pub mod element;
pub mod emphasis;
pub mod fixed_width;
pub mod fn_def;
pub mod fn_ref;
pub mod headline;
pub mod inline_call;
pub mod inline_src;
pub mod input;
pub mod keyword;
pub mod link;
pub mod list;
pub mod macros;
pub mod object;
pub mod paragraph;
pub mod planning;
pub mod radio_target;
pub mod rule;
pub mod snippet;
pub mod table;
pub mod target;
pub mod timestamp;

use rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrgLanguage;

impl Language for OrgLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        //  SAFETY: SyntaxKind is `repr(u16)`
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

pub type SyntaxNode = rowan::SyntaxNode<OrgLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<OrgLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<OrgLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<OrgLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<OrgLanguage>;

#[allow(bad_style)]
#[allow(clippy::all)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    //
    // token
    //
    L_BRACKET,    // '['
    R_BRACKET,    // ']'
    L_BRACKET2,   // '[['
    R_BRACKET2,   // ']]'
    L_PARENS,     // '('
    R_PARENS,     // ')'
    L_ANGLE,      // '<'
    R_ANGLE,      // '>'
    L_CURLY,      // '{'
    R_CURLY,      // '}'
    L_CURLY3,     // '{{{'
    R_CURLY3,     // '}}}'
    L_ANGLE2,     // '<<'
    R_ANGLE2,     // '>>'
    L_ANGLE3,     // '<<<'
    R_ANGLE3,     // '>>>'
    AT,           // '@'
    AT2,          // '@@'
    PERCENT,      // '%'
    PERCENT2,     // '%%'
    SLASH,        // '/'
    UNDERSCORE,   // '_'
    STAR,         // '*'
    PLUS,         // '+'
    MINUS,        // '-'
    MINUS2,       // '--'
    COLON,        // ':'
    COLON2,       // '::'
    EQUAL,        // '='
    TILDE,        // '~'
    HASH,         // '#'
    HASH_PLUS,    // '#+'
    DOUBLE_ARROW, // '=>'
    PIPE,         // '|'
    COMMA,        // ','
    NEW_LINE,     // '\n' or '\r\n' or '\r'
    WHITESPACE,   // ' ' or '\t'
    BLANK_LINE,
    TEXT,

    DOCUMENT,
    SECTION,
    PARAGRAPH,

    HEADLINE,
    HEADLINE_STARS,
    HEADLINE_TITLE,
    HEADLINE_KEYWORD,
    HEADLINE_PRIORITY,
    HEADLINE_TAGS,
    PROPERTY_DRAWER,
    NODE_PROPERTY,
    PLANNING,
    PLANNING_DEADLINE,
    PLANNING_SCHEDULED,
    PLANNING_CLOSED,

    //
    // elements
    //
    /* table */
    ORG_TABLE,
    ORG_TABLE_RULE_ROW,
    ORG_TABLE_STANDARD_ROW,
    ORG_TABLE_CELL,
    /* list */
    LIST,
    LIST_ITEM,
    LIST_ITEM_INDENT,
    LIST_ITEM_BULLET,
    LIST_ITEM_COUNTER,
    LIST_ITEM_CHECK_BOX,
    LIST_ITEM_TAG,
    LIST_ITEM_CONTENT,
    /* drawer */
    DRAWER,
    DRAWER_BEGIN,
    DRAWER_END,
    KEYWORD,
    BABEL_CALL,
    AFFILIATED_KEYWORD,
    TABLE_EL,
    CLOCK,
    FN_DEF,
    COMMENT,
    RULE,
    FIXED_WIDTH,
    /* dyn block */
    DYN_BLOCK,
    DYN_BLOCK_BEGIN,
    DYN_BLOCK_END,
    /* block */
    SPECIAL_BLOCK,
    QUOTE_BLOCK,
    CENTER_BLOCK,
    VERSE_BLOCK,
    COMMENT_BLOCK,
    EXAMPLE_BLOCK,
    EXPORT_BLOCK,
    SOURCE_BLOCK,
    SOURCE_BLOCK_LANG,
    BLOCK_BEGIN,
    BLOCK_END,
    BLOCK_CONTENT,

    //
    // objects
    //
    INLINE_CALL,
    INLINE_SRC,
    LINK,
    LINK_PATH,
    COOKIE,
    RADIO_TARGET,
    FN_REF,
    LATEX_ENVIRONMENT,
    MACROS,
    MACROS_ARGUMENT,
    SNIPPET,
    TARGET,
    BOLD,
    STRIKE,
    ITALIC,
    UNDERLINE,
    VERBATIM,
    CODE,

    /* timestamp */
    TIMESTAMP_ACTIVE,
    TIMESTAMP_INACTIVE,
    TIMESTAMP_DIARY,
    TIMESTAMP_YEAR,
    TIMESTAMP_MONTH,
    TIMESTAMP_DAY,
    TIMESTAMP_HOUR,
    TIMESTAMP_MINUTE,
    TIMESTAMP_DAYNAME,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(value: SyntaxKind) -> Self {
        OrgLanguage::kind_to_raw(value)
    }
}

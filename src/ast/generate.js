const nodes = [
  {
    struct: "Document",
    kind: ["DOCUMENT"],
    pre_blank: true,
    first_child: [
      ["section", "Section"],
      ["first_headline", "Headline"],
    ],
    last_child: [["last_headline", "Headline"]],
    children: [["headlines", "Headline"]],
  },
  {
    struct: "Section",
    kind: ["SECTION"],
    post_blank: true,
  },
  {
    struct: "Paragraph",
    kind: ["PARAGRAPH"],
    post_blank: true,
    affiliated_keywords: true,
  },
  {
    struct: "Headline",
    kind: ["HEADLINE"],
    first_child: [
      ["section", "Section"],
      ["planning", "Planning"],
      ["properties", "PropertyDrawer"],
    ],
    children: [["headlines", "Headline"]],
    post_blank: true,
  },
  {
    struct: "PropertyDrawer",
    kind: ["PROPERTY_DRAWER"],
    children: [["node_properties", "NodeProperty"]],
  },
  {
    struct: "NodeProperty",
    kind: ["NODE_PROPERTY"],
  },
  {
    struct: "Planning",
    kind: ["PLANNING"],
  },
  {
    struct: "OrgTable",
    kind: ["ORG_TABLE"],
    post_blank: true,
    affiliated_keywords: true,
  },
  {
    struct: "OrgTableRow",
    kind: ["ORG_TABLE_RULE_ROW", "ORG_TABLE_STANDARD_ROW"],
  },
  {
    struct: "OrgTableCell",
    kind: ["ORG_TABLE_CELL"],
  },
  {
    struct: "List",
    kind: ["LIST"],
    children: [["items", "ListItem"]],
    affiliated_keywords: true,
  },
  {
    struct: "ListItem",
    kind: ["LIST_ITEM"],
  },
  {
    struct: "Drawer",
    kind: ["DRAWER"],
  },
  {
    struct: "DynBlock",
    kind: ["DYN_BLOCK"],
    affiliated_keywords: true,
  },
  {
    struct: "Keyword",
    kind: ["KEYWORD"],
  },
  {
    struct: "BabelCall",
    kind: ["BABEL_CALL"],
  },
  {
    struct: "AffiliatedKeyword",
    kind: ["AFFILIATED_KEYWORD"],
  },
  {
    struct: "TableEl",
    kind: ["TABLE_EL"],
    post_blank: true,
  },
  {
    struct: "Clock",
    kind: ["CLOCK"],
    post_blank: true,
  },
  {
    struct: "FnDef",
    kind: ["FN_DEF"],
    token: [
      ["label", "FN_LABEL"],
      ["description", "FN_CONTENT"],
    ],
    post_blank: true,
    affiliated_keywords: true,
  },
  {
    struct: "FnContent",
    kind: ["FN_CONTENT"],
    post_blank: true,
    affiliated_keywords: true,
  },
  {
    struct: "Comment",
    kind: ["COMMENT"],
    post_blank: true,
    token: [["text", "TEXT"]],
    affiliated_keywords: true,
  },
  {
    struct: "Rule",
    kind: ["RULE"],
    post_blank: true,
  },
  {
    struct: "FixedWidth",
    kind: ["FIXED_WIDTH"],
    post_blank: true,
    token: [["text", "TEXT"]],
    affiliated_keywords: true,
  },
  {
    struct: "SpecialBlock",
    kind: ["SPECIAL_BLOCK"],
    affiliated_keywords: true,
  },
  {
    struct: "QuoteBlock",
    kind: ["QUOTE_BLOCK"],
    affiliated_keywords: true,
  },
  {
    struct: "CenterBlock",
    kind: ["CENTER_BLOCK"],
    affiliated_keywords: true,
  },
  {
    struct: "VerseBlock",
    kind: ["VERSE_BLOCK"],
    affiliated_keywords: true,
  },
  {
    struct: "CommentBlock",
    kind: ["COMMENT_BLOCK"],
    affiliated_keywords: true,
  },
  {
    struct: "ExampleBlock",
    kind: ["EXAMPLE_BLOCK"],
    affiliated_keywords: true,
  },
  {
    struct: "ExportBlock",
    kind: ["EXPORT_BLOCK"],
    affiliated_keywords: true,
  },
  {
    struct: "SourceBlock",
    kind: ["SOURCE_BLOCK"],
    affiliated_keywords: true,
  },
  {
    struct: "InlineCall",
    kind: ["INLINE_CALL"],
  },
  {
    struct: "InlineSrc",
    kind: ["INLINE_SRC"],
  },
  {
    struct: "Link",
    kind: ["LINK"],
  },
  {
    struct: "Cookie",
    kind: ["COOKIE"],
  },
  {
    struct: "RadioTarget",
    kind: ["RADIO_TARGET"],
  },
  {
    struct: "FnRef",
    kind: ["FN_REF"],
    token: [["label", "FN_LABEL"]],
  },
  {
    struct: "Macros",
    kind: ["MACROS"],
  },
  {
    struct: "Snippet",
    kind: ["SNIPPET"],
  },
  {
    struct: "Target",
    kind: ["TARGET"],
  },
  {
    struct: "Bold",
    kind: ["BOLD"],
  },
  {
    struct: "Strike",
    kind: ["STRIKE"],
  },
  {
    struct: "Italic",
    kind: ["ITALIC"],
  },
  {
    struct: "Underline",
    kind: ["UNDERLINE"],
  },
  {
    struct: "Verbatim",
    kind: ["VERBATIM"],
  },
  {
    struct: "Code",
    kind: ["CODE"],
    token: [["text", "TEXT"]],
  },
  {
    struct: "Timestamp",
    kind: ["TIMESTAMP_ACTIVE", "TIMESTAMP_INACTIVE", "TIMESTAMP_DIARY"],
    token: [
      ["year_start", "TIMESTAMP_YEAR"],
      ["month_start", "TIMESTAMP_MONTH"],
      ["day_start", "TIMESTAMP_DAY"],
      ["hour_start", "TIMESTAMP_HOUR"],
      ["minute_start", "TIMESTAMP_MINUTE"],
    ],
    last_token: [
      ["year_end", "TIMESTAMP_YEAR"],
      ["month_end", "TIMESTAMP_MONTH"],
      ["day_end", "TIMESTAMP_DAY"],
      ["hour_end", "TIMESTAMP_HOUR"],
      ["minute_end", "TIMESTAMP_MINUTE"],
    ],
  },
  {
    struct: "LatexEnvironment",
    kind: ["LATEX_ENVIRONMENT"],
  },
  {
    struct: "LatexFragment",
    kind: ["LATEX_FRAGMENT"],
  },
  {
    struct: "Entity",
    kind: ["ENTITY"],
  },
  {
    struct: "LineBreak",
    kind: ["LINE_BREAK"],
  },
  {
    struct: "Superscript",
    kind: ["SUPERSCRIPT"],
  },
  {
    struct: "Subscript",
    kind: ["SUBSCRIPT"],
  },
];

let content = `//! generated file, do not modify it directly
#![allow(clippy::all)]
#![allow(unused)]

use rowan::{ast::{support, AstChildren, AstNode}, TextSize, TextRange};
use crate::syntax::{OrgLanguage, SyntaxKind, SyntaxKind::*, SyntaxNode, SyntaxToken};

fn affiliated_keyword(node: &SyntaxNode, filter: impl Fn(&str) -> bool) -> Option<AffiliatedKeyword> {
    node.children()
        .take_while(|n| n.kind() == SyntaxKind::AFFILIATED_KEYWORD)
        .filter_map(AffiliatedKeyword::cast)
        .find(|k| filter(&k.key()))
}
`;

for (const node of nodes) {
  content += `
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ${node.struct} {
    pub(crate) syntax: SyntaxNode,
}
impl AstNode for ${node.struct} {
    type Language = OrgLanguage;
    fn can_cast(kind: SyntaxKind) -> bool { ${node.kind
      .map((k) => `kind == ${k}`)
      .join(" || ")} }
    fn cast(node: SyntaxNode) -> Option<${
      node.struct
    }> { Self::can_cast(node.kind()).then(|| ${node.struct} { syntax: node }) }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl ${node.struct} {
    /// Beginning position of this element
    pub fn start(&self) -> TextSize {
        self.syntax.text_range().start()
    }
    /// Ending position of this element
    pub fn end(&self) -> TextSize {
        self.syntax.text_range().end()
    }
    /// Range of this element
    pub fn text_range(&self) -> TextRange {
        self.syntax.text_range()
    }
    /// Raw text of this element
    pub fn raw(&self) -> String {
        self.syntax.to_string()
    }
`;
  for (const [method, kind] of node.token || []) {
    content += `    pub fn ${method}(&self) -> Option<super::Token> { super::token(&self.syntax, ${kind}) }\n`;
  }
  for (const [method, kind] of node.last_token || []) {
    content += `    pub fn ${method}(&self) -> Option<super::Token> { super::last_token(&self.syntax, ${kind}) }\n`;
  }
  for (const [method, kind] of node.parent || []) {
    content += `    pub fn ${method}(&self) -> Option<${kind}> { self.syntax.parent().and_then(${kind}::cast) }\n`;
  }
  for (const [method, kind] of node.first_child || []) {
    content += `    pub fn ${method}(&self) -> Option<${kind}> { support::child(&self.syntax) }\n`;
  }
  for (const [method, kind] of node.last_child || []) {
    content += `    pub fn ${method}(&self) -> Option<${kind}> { super::last_child(&self.syntax) }\n`;
  }
  for (const [method, kind] of node.children || []) {
    content += `    pub fn ${method}(&self) -> AstChildren<${kind}> { support::children(&self.syntax) }\n`;
  }
  if (node.post_blank) {
    content += `    pub fn post_blank(&self) -> usize { super::blank_lines(&self.syntax) }\n`;
  }
  if (node.pre_blank) {
    content += `    pub fn pre_blank(&self) -> usize { super::blank_lines(&self.syntax) }\n`;
  }
  if (node.affiliated_keywords) {
    content += `    pub fn caption(&self) -> Option<AffiliatedKeyword> { affiliated_keyword(&self.syntax, |k| k == "CAPTION") }\n`;
    content += `    pub fn header(&self) -> Option<AffiliatedKeyword> { affiliated_keyword(&self.syntax, |k| k == "HEADER") }\n`;
    content += `    pub fn name(&self) -> Option<AffiliatedKeyword> { affiliated_keyword(&self.syntax, |k| k == "NAME") }\n`;
    content += `    pub fn plot(&self) -> Option<AffiliatedKeyword> { affiliated_keyword(&self.syntax, |k| k == "PLOT") }\n`;
    content += `    pub fn results(&self) -> Option<AffiliatedKeyword> { affiliated_keyword(&self.syntax, |k| k == "RESULTS") }\n`;
    content += `    pub fn attr(&self, backend: &str) -> Option<AffiliatedKeyword> { affiliated_keyword(&self.syntax, |k| k.starts_with("ATTR_") && &k[5..] == backend) }\n`;
  }
  content += `}\n`;
}

require("fs").writeFileSync(__dirname + "/generated.rs", content);

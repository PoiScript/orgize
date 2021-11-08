export class Handler {
  text(_text: string) {}
  code(_item: string) {}
  cookie(_item: Cookie) {}
  rule() {}
  exampleBlock(_item: Block) {}
  exportBlock(_item: Block) {}
  sourceBlock(_item: SourceBlock) {}
  inlineSrc(_item: InlineSrc) {}
  link(_item: Link) {}
  snippet(_item: Snippet) {}
  timestamp(_item: any) {}
  verbatim(_item: string) {}
  fixedWidth(_item: FixedWidth) {}
  listStart(_item: List) {}
  listEnd(_item: List) {}
  tableStart(_item: any) {}
  tableEnd(_item: any) {}
  tableRowStart(_item: any) {}
  tableRowEnd(_item: any) {}
  tableCellStart(_item: any) {}
  tableCellEnd(_item: any) {}
  titleStart(_item: Title) {}
  titleEnd(_item: Title) {}
  boldStart() {}
  boldEnd() {}
  centerBlockStart(_item: any) {}
  centerBlockEnd(_item: any) {}
  documentStart() {}
  documentEnd() {}
  italicStart() {}
  italicEnd() {}
  listItemStart() {}
  listItemEnd() {}
  paragraphStart() {}
  paragraphEnd() {}
  quoteBlockStart(_item: any) {}
  quoteBlockEnd(_item: any) {}
  sectionStart() {}
  sectionEnd() {}
  strikeStart() {}
  strikeEnd() {}
  underlineStart() {}
  underlineEnd() {}
  verseBlockStart(_item: any) {}
  verseBlockEnd(_item: any) {}
  keyword(_item: Keyword) {}
}

export type Title = {
  level: number;
  priority?: string;
  tags?: string[];
  keyword?: string;
  raw: string;
  properties?: { [key: string]: string };
  post_blank: number;
};

export type List = {
  ordered: boolean;
};

export type Block = {
  contents: string;
};

export type InlineSrc = {
  lang: string;
  body: string;
};

export type Link = {
  path: string;
  desc?: string;
};

export type FixedWidth = {
  value: string;
};

export type Cookie = {
  value: string;
};

export type SourceBlock = {
  contents: string;
  language: string;
  arguments: string;
  post_blank: number;
};

export type Keyword = {
  key: string;
  optional?: string;
  value: string;
};

export type Snippet = {
  name: string;
  value: string;
};

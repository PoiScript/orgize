import {
  Block,
  Cookie,
  FixedWidth,
  Handler,
  InlineSrc,
  Link,
  List,
  Snippet,
  Title,
} from "./handler";

const tags: { [tag: string]: string } = {
  "&": "&amp;",
  "<": "&lt;",
  ">": "&gt;",
  '"': "&quot;",
  "'": "&apos;",
};

const replaceTags = (tag: string): string => tags[tag];

export const escapeHtml = (str: string): string =>
  str.replace(/[&<>"']/g, replaceTags);

export class HtmlHandler extends Handler {
  result: string;

  constructor(result: string = "") {
    super();
    this.result = result;
  }

  static escape(): string {
    return "";
  }

  quoteBlockStart() {
    this.result += "<blockquote>";
  }
  quoteBlockEnd() {
    this.result += "</blockquote>";
  }
  centerBlockStart() {
    this.result += '<div class="center">';
  }
  centerBlockEnd() {
    this.result += "</div>";
  }
  verseBlockStart() {
    this.result += '<p class="verse">';
  }
  verseBlockEnd() {
    this.result += "</p>";
  }
  boldStart() {
    this.result += "<b>";
  }
  boldEnd() {
    this.result += "</b>";
  }
  documentStart() {
    this.result += "<main>";
  }
  documentEnd() {
    this.result += "</main>";
  }

  listStart(list: List) {
    this.result += `<${list.ordered ? "o" : "u"}l>`;
  }
  listEnd(list: List) {
    this.result += `</${list.ordered ? "o" : "u"}l>`;
  }

  italicStart() {
    this.result += "<i>";
  }
  italicEnd() {
    this.result += "</i>";
  }
  listItemStart() {
    this.result += "<li>";
  }
  listItemEnd() {
    this.result += "</li>";
  }
  paragraphStart() {
    this.result += "<p>";
  }
  paragraphEnd() {
    this.result += "</p>";
  }
  sectionStart() {
    this.result += "<section>";
  }
  sectionEnd() {
    this.result += "</section>";
  }
  strikeStart() {
    this.result += "<s>";
  }
  strikeEnd() {
    this.result += "</s>";
  }
  underlineStart() {
    this.result += "<u>";
  }
  underlineEnd() {
    this.result += "</u>";
  }

  exampleBlock(block: Block) {
    this.result += `<pre class="example">${escapeHtml(block.contents)}</pre>`;
  }

  sourceBlock(block: Block) {
    this.result += `<pre class="example">${escapeHtml(block.contents)}</pre>`;
  }
  inlineSrc(src: InlineSrc) {
    this.result += `<code class="src src-${src.lang}">${escapeHtml(
      src.body
    )}</code>`;
  }
  code(value: string) {
    this.result += `<code>${escapeHtml(value)}</code>`;
  }
  link(link: Link) {
    this.result += `<a href="${link.path}">${escapeHtml(
      link.desc || link.path
    )}</a>`;
  }
  snippet(snippet: Snippet) {
    if (snippet.name.toLowerCase() === "html") {
      this.result += snippet.value;
    }
  }
  text(value: string) {
    this.result += escapeHtml(value);
  }
  verbatim(value: string) {
    this.result += `<code>${escapeHtml(value)}</code>`;
  }
  fixedWidth(item: FixedWidth) {
    this.result += `<pre class="example">${escapeHtml(item.value)}</pre>`;
  }
  rule() {
    this.result += "<hr>";
  }
  cookie(cookie: Cookie) {
    this.result += `<code>${escapeHtml(cookie.value)}</code>`;
  }

  titleStart(title: Title) {
    this.result += `<h${Math.min(title.level, 6)}>`;
  }
  titleEnd(title: Title) {
    this.result += `</h${Math.min(title.level, 6)}>`;
  }
}

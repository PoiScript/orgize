import { Handler, Keyword } from "./handler";

export class CollectKeywords extends Handler {
  keywords: { [key: string]: string[] } = {};

  keyword(keyword: Keyword) {
    this.keywords[keyword.key] = this.keywords[keyword.key] || [];
    this.keywords[keyword.key].push(keyword.value);
  }
}

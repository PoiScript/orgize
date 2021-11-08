import init, {
  handle as internalHandle,
  InitInput,
  InitOutput,
  Org,
} from "../pkg/orgize";
import { Handler } from "./handler";
import { HtmlHandler } from "./html";
import { CollectKeywords } from "./keyword";

export const handle = (org: Org | string, handler: Handler) => {
  if (typeof org === "string") {
    org = Org.parse(org);
  }
  internalHandle(org, handler);
};

export const renderHtml = (
  org: Org | string,
  handler: HtmlHandler = new HtmlHandler()
): string => {
  handle(org, handler);
  return handler.result;
};

export const keywords = (org: Org | string): { [key: string]: string[] } => {
  const handler = new CollectKeywords();
  handle(org, handler);
  return handler.keywords;
};

export * from "./handler";
export * from "./html";
export { Org, init, InitInput, InitOutput };

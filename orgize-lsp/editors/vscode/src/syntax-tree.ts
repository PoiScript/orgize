import {
  Disposable,
  TextDocumentContentProvider,
  Uri,
  commands,
  window,
  workspace,
} from "vscode";
import { client } from "./main";

export default class SyntaxTreeProvider implements TextDocumentContentProvider {
  static readonly scheme = "orgize-syntax-tree";

  static register(): Disposable {
    const provider = new SyntaxTreeProvider();

    // register content provider for scheme `references`
    // register document link provider for scheme `references`
    const providerRegistrations = workspace.registerTextDocumentContentProvider(
      SyntaxTreeProvider.scheme,
      provider
    );

    // register command that crafts an uri with the `references` scheme,
    // open the dynamic document, and shows it in the next editor
    const commandRegistration = commands.registerTextEditorCommand(
      "orgize.syntax-tree",
      (editor) => {
        return workspace
          .openTextDocument(encode(editor.document.uri))
          .then((doc) => window.showTextDocument(doc, editor.viewColumn! + 1));
      }
    );

    return Disposable.from(
      provider,
      commandRegistration,
      providerRegistrations
    );
  }

  dispose() {
    // this._subscriptions.dispose();
    // this._documents.clear();
    // this._editorDecoration.dispose();
    // this._onDidChange.dispose();
  }

  async provideTextDocumentContent(uri: Uri): Promise<string> {
    if (!client) {
      return "LSP server is not ready...";
    }

    const result = await client.sendRequest("workspace/executeCommand", {
      command: "orgize.syntax-tree",
      arguments: [decode(uri).toString()],
    });

    if (typeof result === "string") {
      return result;
    }

    return "";
  }
}

const encode = (uri: Uri): Uri => {
  return uri.with({
    scheme: SyntaxTreeProvider.scheme,
    query: uri.path,
    path: "tree.syntax",
  });
};

const decode = (uri: Uri): Uri => {
  return uri.with({ scheme: "file", path: uri.query, query: "" });
};

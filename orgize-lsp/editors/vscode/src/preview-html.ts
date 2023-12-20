import {
  Disposable,
  ExtensionContext,
  TextDocumentContentProvider,
  Uri,
  ViewColumn,
  Webview,
  WebviewOptions,
  WebviewPanel,
  commands,
  window,
  workspace,
} from "vscode";

import { client } from "./main";

export const register = (context: ExtensionContext) => {
  const provider = new PreviewHtmlProvider();

  context.subscriptions.push(
    workspace.registerTextDocumentContentProvider(
      "orgize-lsp-preview",
      provider
    )
  );
};

export default class PreviewHtmlProvider
  implements TextDocumentContentProvider
{
  static readonly scheme = "orgize-preview-html";

  static register(): Disposable {
    const provider = new PreviewHtmlProvider();

    // register content provider for scheme `references`
    // register document link provider for scheme `references`
    const providerRegistrations = workspace.registerTextDocumentContentProvider(
      PreviewHtmlProvider.scheme,
      provider
    );

    // register command that crafts an uri with the `references` scheme,
    // open the dynamic document, and shows it in the next editor
    const commandRegistration = commands.registerTextEditorCommand(
      "orgize.preview-html",
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

    return client.sendRequest("workspace/executeCommand", {
      command: "orgize.syntax-tree",
      arguments: [uri.toString()],
    });
  }
}

class PreviewHtmlPanel {
  /**
   * Track the currently panel. Only allow a single panel to exist at a time.
   */
  public static currentPanel: PreviewHtmlPanel | undefined;

  public static readonly viewType = "orgizePreviewHtml";

  private readonly _panel: WebviewPanel;
  private readonly _extensionUri: Uri;
  private _disposables: Disposable[] = [];

  public static createOrShow(uri: Uri) {
    const column = window.activeTextEditor
      ? window.activeTextEditor.viewColumn
      : undefined;

    // If we already have a panel, show it.
    if (PreviewHtmlPanel.currentPanel) {
      PreviewHtmlPanel.currentPanel._panel.reveal(column);
      return;
    }

    // Otherwise, create a new panel.
    const panel = window.createWebviewPanel(
      PreviewHtmlPanel.viewType,
      "Preview of " + uri.fsPath,
      column || ViewColumn.One,
      getWebviewOptions(uri)
    );

    PreviewHtmlPanel.currentPanel = new PreviewHtmlPanel(panel, uri);
  }

  public static revive(panel: WebviewPanel, extensionUri: Uri) {
    PreviewHtmlPanel.currentPanel = new PreviewHtmlPanel(panel, extensionUri);
  }

  private constructor(panel: WebviewPanel, extensionUri: Uri) {
    this._panel = panel;
    this._extensionUri = extensionUri;

    // Set the webview's initial html content
    this._update();

    // Listen for when the panel is disposed
    // This happens when the user closes the panel or when the panel is closed programmatically
    this._panel.onDidDispose(
      () => {
        this.dispose();
      },
      null,
      this._disposables
    );

    // Update the content based on view changes
    this._panel.onDidChangeViewState(
      (e) => {
        if (this._panel.visible) {
          this._update();
        }
      },
      null,
      this._disposables
    );
  }

  public dispose() {
    PreviewHtmlPanel.currentPanel = undefined;

    // Clean up our resources
    this._panel.dispose();

    while (this._disposables.length) {
      const x = this._disposables.pop();
      if (x) {
        x.dispose();
      }
    }
  }

  private _update() {
    const webview = this._panel.webview;
    this._panel.webview.html = this._getHtmlForWebview(webview);
  }

  private _getHtmlForWebview(webview: Webview): string {
    // // Local path to main script run in the webview
    // const scriptPathOnDisk = Uri.joinPath(
    //   this._extensionUri,
    //   "media",
    //   "main.js"
    // );

    // // And the uri we use to load this script in the webview
    // const scriptUri = webview.asWebviewUri(scriptPathOnDisk);

    // // Local path to css styles
    // const styleResetPath = Uri.joinPath(
    //   this._extensionUri,
    //   "media",
    //   "reset.css"
    // );

    // const stylesPathMainPath = Uri.joinPath(
    //   this._extensionUri,
    //   "media",
    //   " css"
    // );

    // // Uri to load styles into webview
    // const stylesResetUri = webview.asWebviewUri(styleResetPath);
    // const stylesMainUri = webview.asWebviewUri(stylesPathMainPath);

    // Use a nonce to only allow specific scripts to be run
    // const nonce = getNonce();

    return `<!DOCTYPE html>
			<html lang="en">
			<head>
				<meta charset="UTF-8">

				<meta name="viewport" content="width=device-width, initial-scale=1.0">

				<title>Cat Coding</title>
			</head>
			<body>
				<img width="300" />
				<h1 id="lines-of-code-counter">0</h1>
			</body>
			</html>`;
  }
}

const getWebviewOptions = (extensionUri: Uri): WebviewOptions => {
  return {
    // Enable javascript in the webview
    enableScripts: true,

    // And restrict the webview to only loading content from our extension's `media` directory.
    localResourceRoots: [Uri.joinPath(extensionUri, "media")],
  };
};

const encode = (uri: Uri): Uri => {
  return uri.with({
    scheme: PreviewHtmlProvider.scheme,
    query: uri.path,
    path: "tree.syntax",
  });
};

const decode = (uri: Uri): Uri => {
  return uri.with({ scheme: "file", path: uri.query, query: "" });
};

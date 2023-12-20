import {
  Disposable,
  ExtensionContext,
  Uri,
  ViewColumn,
  WebviewPanel,
  commands,
  window,
  workspace,
} from "vscode";
import { Utils } from "vscode-uri";

import { client } from "./main";

export const register = (context: ExtensionContext) => {
  context.subscriptions.push(
    commands.registerTextEditorCommand("orgize.preview-html", (editor) => {
      PreviewHtmlPanel.createOrShow(context.extensionUri, editor.document.uri);
    })
  );
};

class PreviewHtmlPanel {
  /**
   * Track the currently panel. Only allow a single panel to exist at a time.
   */
  public static currentPanel: PreviewHtmlPanel | undefined;

  public static readonly viewType = "orgizePreviewHtml";

  private readonly _panel: WebviewPanel;
  private _orgUri: Uri;
  private readonly _extensionUri: Uri;

  private _disposables: Disposable[] = [];

  public static createOrShow(extensionUri: Uri, orgUri: Uri) {
    const column = window.activeTextEditor.viewColumn! + 1;

    // If we already have a panel, show it.
    if (PreviewHtmlPanel.currentPanel) {
      PreviewHtmlPanel.currentPanel._panel.reveal(column);
      PreviewHtmlPanel.currentPanel._orgUri = orgUri;
      PreviewHtmlPanel.currentPanel.refresh();
      return;
    }

    // Otherwise, create a new panel.
    const panel = window.createWebviewPanel(
      PreviewHtmlPanel.viewType,
      "Preview of " + Utils.basename(orgUri),
      column || ViewColumn.One,
      {
        // Enable javascript in the webview
        enableScripts: true,

        // And restrict the webview to only loading content from our extension's `media` directory.
        localResourceRoots: [
          Uri.joinPath(extensionUri, "media"),
          ...workspace.workspaceFolders.map((folder) => folder.uri),
        ],
      }
    );

    PreviewHtmlPanel.currentPanel = new PreviewHtmlPanel(
      panel,
      extensionUri,
      orgUri
    );
  }

  private constructor(panel: WebviewPanel, extensionUri: Uri, orgUri: Uri) {
    this._panel = panel;
    this._orgUri = orgUri;
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

    workspace.onDidChangeTextDocument((event) => {
      if (event.document.uri.fsPath === this._orgUri.fsPath) {
        this.refresh();
      }
    }, this._disposables);

    workspace.onDidOpenTextDocument((document) => {
      if (document.uri.fsPath === this._orgUri.fsPath) {
        this.refresh();
      }
    }, this._disposables);

    // Update the content based on view changes
    this._panel.onDidChangeViewState(
      (e) => {
        if (this._panel.visible) {
          this.refresh();
        }
      },
      null,
      this._disposables
    );
  }

  private readonly _delay = 300;
  private _throttleTimer: any;
  private _firstUpdate = true;

  public refresh() {
    // Schedule update if none is pending
    if (!this._throttleTimer) {
      if (this._firstUpdate) {
        this._update();
      } else {
        this._throttleTimer = setTimeout(() => this._update(), this._delay);
      }
    }

    this._firstUpdate = false;
  }

  private async _update() {
    clearTimeout(this._throttleTimer);
    this._throttleTimer = undefined;

    if (!client) {
      return;
    }

    try {
      const content: string = await client.sendRequest(
        "workspace/executeCommand",
        {
          command: "orgize.preview-html",
          arguments: [this._orgUri.with({ scheme: "file" }).toString()],
        }
      );
      this._panel.webview.html = this._makeHtml(content);
    } catch {}
  }

  private _makeHtml(content: string): string {
    const stylesPath = Uri.joinPath(
      this._extensionUri,
      "media",
      "org-mode.css"
    );

    return `<!doctype html>
      <html lang="en">
        <head>
          <meta charset="UTF-8" />

          <meta
            name="viewport"
            content="width=device-width, initial-scale=1.0"
          />

          <base
            href="${this._panel.webview.asWebviewUri(this._orgUri)}"
          />

          <link
            href="${this._panel.webview.asWebviewUri(stylesPath)}"
            rel="stylesheet"
          />
        </head>
        <body>
          ${content}
        </body>
      </html>`;
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
}

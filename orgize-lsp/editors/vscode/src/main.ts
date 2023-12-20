import { ExtensionContext } from "vscode";

import {
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

import SyntaxTreeProvider from "./syntax-tree";
import { register } from "./preview-html";

export let client: LanguageClient;

export function activate(context: ExtensionContext) {
  // If the extension is launched in debug mode then the debug server options are used
  // Otherwise the run options are used
  const run: Executable = {
    command: "orgize-lsp",
  };

  const serverOptions: ServerOptions = {
    run,
    debug: run,
  };

  // Options to control the language client
  const clientOptions: LanguageClientOptions = {
    // Register the server for plain text documents
    documentSelector: [{ scheme: "file", language: "org" }],
  };

  // Create the language client and start the client.
  client = new LanguageClient(
    "orgize-lsp",
    "Orgize LSP",
    serverOptions,
    clientOptions
  );

  // Start the client. This will also launch the server
  client.start();

  context.subscriptions.push(SyntaxTreeProvider.register());
  register(context);
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}

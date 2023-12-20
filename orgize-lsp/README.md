# `orgize-lsp`

Language server for org-mode, powered by orgize.

## Install

### Server

```sh
$ cargo install --path .
```

### Client (vscode)

```sh
$ pnpm run -C editors/vscode package --no-dependencies
$ code --install-extension ./editors/vscode/orgize-lsp.vsix --force
```

## Supported features

1. Folding range

   - Fold headline, list, table, blocks

2. Document symbols

   - Headings

3. Formatting

4. Document link

   - File links

   - Source block `:tangle` arguments

   - Internal links

5. Code lens

   - Generate toc heading

   - Tangle/detanlge source block

   - Evaluate source block

6. Commands

   - Show syntax tree

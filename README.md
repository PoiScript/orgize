# Orgize

[![Build status](https://img.shields.io/github/actions/workflow/status/PoiScript/orgize/ci.yml)](https://github.com/PoiScript/orgize/actions/workflows/ci.yml)
![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)

Org-mode toolkit written in Rust.

This repository contains several crates/packages:

| Crates/packages               | Description                                                     |
| ----------------------------- | --------------------------------------------------------------- |
| [`orgize`]                    | A pure-rust library for parsing and exporting org-mode files.   |
| [`orgize-cli`]                | Command line utilities for org-mode files, builtin with orgize. |
| [`orgize-lsp`]                | Language server for org-mode files, builtin with orgize.        |
| [`orgize-lsp/editors/vscode`] | [`orgize-lsp`] client for vscode editor                         |
| [`orgize-common`]             | Shared code for [`orgize-cli`] and [`orgize-lsp`].              |
| [`orgize-wasm`]               | WebAssembly module for Browser or Node.js environment.          |

[`orgize`]: ./orgize
[`orgize-cli`]: ./orgize-cli
[`orgize-lsp`]: ./orgize-lsp
[`orgize-lsp/editors/vscode`]: ./orgize-lsp/editors/vscode
[`orgize-common`]: ./orgize-common
[`orgize-wasm`]: ./orgize-wasm

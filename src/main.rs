use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct RustfmtLsp {
    client: Client,
    documents: Mutex<HashMap<Url, String>>,
}

#[derive(Clone)]
struct KeyInfo {
    name: &'static str,
    doc: &'static str,
    default: &'static str,
    values: Vec<&'static str>, // empty = freeform (number/string), non-empty = enum
    kind: ValueKind,
}

#[derive(Clone)]
enum ValueKind {
    Bool,
    Integer,
    String,
    Enum,
}

fn all_keys() -> Vec<KeyInfo> {
    vec![
        KeyInfo {
            name: "array_width",
            doc: "Maximum width of an array literal before falling back to vertical formatting.",
            default: "60",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "attr_fn_like_width",
            doc: "Maximum width of the args of a function-like attributes before falling back to vertical formatting.",
            default: "70",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "binop_seperator",
            doc: "Where to put a binary operator when a binary expression goes multiline.",
            default: "Front",
            values: vec!["Front", "Back"],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "newline_style",
            doc: "Unix or Windows line endings.",
            default: "\"Auto\"",
            values: vec!["\"Auto\"", "\"Native\"", "\"Unix\"", "\"Windows\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "indent_style",
            doc: "Indent on expressions or items.",
            default: "\"Block\"",
            values: vec!["\"Block\"", "\"Visual\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "use_small_heuristics",
            doc: "Whether to use different formatting for items and expressions if they satisfy a heuristic notion of 'small'.",
            default: "\"Default\"",
            values: vec!["\"Default\"", "\"Off\"", "\"Max\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "edition",
            doc: "The Rust edition to use for parsing.",
            default: "\"2015\"",
            values: vec!["\"2015\"", "\"2018\"", "\"2021\"", "\"2024\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "merge_derives",
            doc: "Merge multiple derives into a single one.",
            default: "true",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "use_try_shorthand",
            doc: "Replace uses of the try! macro by the ? shorthand.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "use_field_init_shorthand",
            doc: "Use field init shorthand if possible.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "reorder_imports",
            doc: "Reorder import and extern crate statements alphabetically.",
            default: "true",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "reorder_modules",
            doc: "Reorder module statements alphabetically in group.",
            default: "true",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "remove_nested_parens",
            doc: "Remove nested parens.",
            default: "true",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "fn_args_layout",
            doc: "Control the layout of arguments in a function.",
            default: "\"Tall\"",
            values: vec!["\"Compressed\"", "\"Tall\"", "\"Vertical\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "force_explicit_abi",
            doc: "Always print the abi for extern items.",
            default: "true",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "format_strings",
            doc: "Format string literals where necessary.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "wrap_comments",
            doc: "Break comments to fit on the line.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "comment_width",
            doc: "Maximum length of comments. No effect unless wrap_comments = true.",
            default: "80",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "normalize_comments",
            doc: "Convert /* */ comments to // comments where possible.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "normalize_doc_attributes",
            doc: "Convert #![doc] and #[doc] attributes to //! and /// doc comments.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "format_code_in_doc_comments",
            doc: "Format code snippet included in doc comments.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "doc_comment_code_block_width",
            doc: "Max width for code snippets included in doc comments. Only used if format_code_in_doc_comments is true.",
            default: "100",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "match_arm_blocks",
            doc: "Wrap the body of arms in blocks when it does not fit on the same line.",
            default: "true",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "match_arm_leading_pipes",
            doc: "Controls whether to include a leading pipe on match arms.",
            default: "\"Never\"",
            values: vec!["\"Always\"", "\"Never\"", "\"Preserve\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "match_block_trailing_comma",
            doc: "Put a trailing comma after a block based match arm (non-block arms are not affected).",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "force_multiline_blocks",
            doc: "Force multiline closure and match arm bodies to be wrapped in a block.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "fn_single_line",
            doc: "Put single-expression functions on a single line.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "where_single_line",
            doc: "Force where-clauses to be on a single line.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "imports_indent",
            doc: "Indent of imports.",
            default: "\"Block\"",
            values: vec!["\"Block\"", "\"Visual\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "imports_layout",
            doc: "Item layout inside a import block.",
            default: "\"Mixed\"",
            values: vec!["\"Horizontal\"", "\"HorizontalVertical\"", "\"Mixed\"", "\"Vertical\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "imports_granularity",
            doc: "How imports should be grouped into use statements.",
            default: "\"Preserve\"",
            values: vec!["\"Preserve\"", "\"Crate\"", "\"Module\"", "\"Item\"", "\"One\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "group_imports",
            doc: "Controls the strategy for how imports are grouped together.",
            default: "\"Preserve\"",
            values: vec!["\"Preserve\"", "\"StdExternalCrate\"", "\"One\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "reorder_impl_items",
            doc: "Reorder impl items. type and const are put first, then macros and methods.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "hex_literal_case",
            doc: "Format the hex literals in expressions.",
            default: "\"Preserve\"",
            values: vec!["\"Preserve\"", "\"Upper\"", "\"Lower\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "empty_item_single_line",
            doc: "Put empty-body functions and impls on a single line.",
            default: "true",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "struct_lit_single_line",
            doc: "Put small struct literals on a single line.",
            default: "true",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "trailing_semicolon",
            doc: "Add trailing semicolon after break, continue and return.",
            default: "true",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "trailing_comma",
            doc: "How to handle trailing commas for lists.",
            default: "\"Vertical\"",
            values: vec!["\"Always\"", "\"Never\"", "\"Vertical\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "overflow_delimited_expr",
            doc: "Allow trailing bracket/parenthesis to overflow after a function call if the last expression is a block-like structure.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "struct_field_align_threshold",
            doc: "The maximum diff of width between struct fields to be aligned with each other.",
            default: "0",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "enum_discrim_align_threshold",
            doc: "The maximum diff of width between enum variants to be aligned with each other.",
            default: "0",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "space_before_colon",
            doc: "Leave a space before the colon.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "space_after_colon",
            doc: "Leave a space after the colon.",
            default: "true",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "spaces_around_ranges",
            doc: "Put spaces around the .. and ..= range operators.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "type_punctuation_density",
            doc: "Determines if + or = are wrapped in spaces in the punctuation of types.",
            default: "\"Wide\"",
            values: vec!["\"Compressed\"", "\"Wide\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "unstable_features",
            doc: "Enable unstable features on the unstable channel.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "disable_all_formatting",
            doc: "Don't reformat anything.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "skip_children",
            doc: "Don't reformat out of line modules.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "show_parse_errors",
            doc: "Show parse errors.",
            default: "true",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "error_on_line_overflow",
            doc: "Error if unable to get all lines within max_width.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "error_on_unformatted",
            doc: "Error if unable to get comments or string literals within max_width, or they are left with trailing whitespace.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "array_width",
            doc: "Maximum width of an array literal before falling back to vertical formatting.",
            default: "60",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "attr_fn_like_width",
            doc: "Maximum width of the args of a function-like attribute macro before falling back to vertical formatting.",
            default: "70",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "chain_width",
            doc: "Maximum width of a chain to fit on one line.",
            default: "60",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "fn_call_width",
            doc: "Maximum width of the args of a function call before falling back to vertical formatting.",
            default: "60",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "single_line_if_else_max_width",
            doc: "Maximum line length for single line if-else expressions. Zero means always break.",
            default: "50",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "single_line_let_else_max_width",
            doc: "Maximum line length for single line let-else statements. Zero means always break.",
            default: "50",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "struct_lit_width",
            doc: "Maximum width in the body of a struct literal before falling back to vertical formatting.",
            default: "18",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "struct_variant_width",
            doc: "Maximum width in the body of a struct variant before falling back to vertical formatting.",
            default: "35",
            values: vec![],
            kind: ValueKind::Integer,
        },
    ]
}

/// Figure out what the user is typing on the current line
fn parse_line_context(line: &str) -> LineContext {
    if let Some(eq_pos) = line.find('=') {
        let key = line[..eq_pos].trim().to_string();
        let after_eq = line[eq_pos + 1..].trim().to_string();
        LineContext::Value { key, partial: after_eq }
    } else {
        LineContext::Key {
            partial: line.trim().to_string(),
        }
    }
}

enum LineContext {
    Key { partial: String },
    Value { key: String, partial: String },
}

/// Get all keys already defined in the document
fn existing_keys(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || !trimmed.contains('=') {
                return None;
            }
            Some(trimmed.split('=').next()?.trim().to_string())
        })
        .collect()
}

#[tower_lsp::async_trait]
impl LanguageServer for RustfmtLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![
                        "=".into(),
                        " ".into(),
                        "\"".into(),
                    ]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "rustfmt-lsp ready")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let mut docs = self.documents.lock().unwrap();
        docs.insert(params.text_document.uri, params.text_document.text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let mut docs = self.documents.lock().unwrap();
        if let Some(change) = params.content_changes.into_iter().last() {
            docs.insert(params.text_document.uri, change.text);
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let mut docs = self.documents.lock().unwrap();
        docs.remove(&params.text_document.uri);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;

        let docs = self.documents.lock().unwrap();
        let doc_text = match docs.get(&uri) {
            Some(text) => text.clone(),
            None => return Ok(None),
        };
        drop(docs);

        let current_line = doc_text
            .lines()
            .nth(pos.line as usize)
            .unwrap_or("");

        let keys = all_keys();
        let used = existing_keys(&doc_text);

        match parse_line_context(current_line) {
            // User is typing a key name
            LineContext::Key { partial } => {
                let items: Vec<CompletionItem> = keys
                    .iter()
                    .filter(|k| !used.contains(&k.name.to_string()))
                    .filter(|k| {
                        partial.is_empty() || k.name.starts_with(&partial)
                    })
                    .map(|k| CompletionItem {
                        label: k.name.to_string(),
                        kind: Some(CompletionItemKind::PROPERTY),
                        detail: Some(format!("Default: {}", k.default)),
                        documentation: Some(Documentation::MarkupContent(
                            MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: k.doc.to_string(),
                            },
                        )),
                        insert_text: Some(format!("{} = {}", k.name, k.default)),
                        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                        // Replace the entire line content before cursor
                        text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                            range: Range {
                                start: Position {
                                    line: pos.line,
                                    character: 0,
                                },
                                end: pos,
                            },
                            new_text: format!("{} = {}", k.name, k.default),
                        })),
                        ..Default::default()
                    })
                    .collect();

                Ok(Some(CompletionResponse::Array(items)))
            }

            // User is typing a value after '='
            LineContext::Value { key, partial } => {
                let key_info = match keys.iter().find(|k| k.name == key) {
                    Some(k) => k,
                    None => return Ok(None),
                };

                // If the key has known enum/bool values, suggest them
                if !key_info.values.is_empty() {
                    let items: Vec<CompletionItem> = key_info
                        .values
                        .iter()
                        .filter(|v| {
                            partial.is_empty() || v.starts_with(&partial)
                        })
                        .map(|v| {
                            let is_default = *v == key_info.default;
                            CompletionItem {
                                label: v.to_string(),
                                kind: Some(CompletionItemKind::VALUE),
                                detail: if is_default {
                                    Some("(default)".to_string())
                                } else {
                                    None
                                },
                                // Replace everything after '= '
                                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                                    range: Range {
                                        start: Position {
                                            line: pos.line,
                                            character: current_line
                                                .find('=')
                                                .map(|i| i as u32 + 2)
                                                .unwrap_or(pos.character),
                                        },
                                        end: Position {
                                            line: pos.line,
                                            character: current_line.len() as u32,
                                        },
                                    },
                                    new_text: v.to_string(),
                                })),
                                ..Default::default()
                            }
                        })
                        .collect();

                    Ok(Some(CompletionResponse::Array(items)))
                } else {
                    // For integer fields, suggest the default
                    Ok(Some(CompletionResponse::Array(vec![CompletionItem {
                        label: key_info.default.to_string(),
                        kind: Some(CompletionItemKind::VALUE),
                        detail: Some("(default)".to_string()),
                        ..Default::default()
                    }])))
                }
            }
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let docs = self.documents.lock().unwrap();
        let doc_text = match docs.get(&uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(docs);

        let line = doc_text.lines().nth(pos.line as usize).unwrap_or("");
        let key_name = line.split('=').next().unwrap_or("").trim();

        let keys = all_keys();
        if let Some(k) = keys.iter().find(|k| k.name == key_name) {
            let mut hover_text = format!("## {}\n\n{}\n\n", k.name, k.doc);
            hover_text.push_str(&format!("**Default:** `{}`\n\n", k.default));

            if !k.values.is_empty() {
                hover_text.push_str("**Possible values:** ");
                let vals: Vec<String> =
                    k.values.iter().map(|v| format!("`{}`", v)).collect();
                hover_text.push_str(&vals.join(", "));
            }

            Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: hover_text,
                }),
                range: None,
            }))
        } else {
            Ok(None)
        }
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| RustfmtLsp {
        client,
        documents: Mutex::new(HashMap::new()),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}

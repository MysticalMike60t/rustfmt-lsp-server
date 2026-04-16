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
    #[allow(warnings)] name: &'static str,
    #[allow(warnings)] doc: &'static str,
    #[allow(warnings)] default: &'static str,
    #[allow(warnings)] values: Vec<&'static str>,
    #[allow(warnings)] kind: ValueKind,
}

#[derive(Clone)]
enum ValueKind {
    #[allow(warnings)] Bool,
    #[allow(warnings)] Integer,
    #[allow(warnings)] String,
    #[allow(warnings)] Enum,
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
            default: "\"Front\"",
            values: vec!["\"Front\"", "\"Back\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "blank_lines_lower_bound",
            doc: "Minimum number of blank lines which must be put between items. If two items have fewer blank lines between them, additional blank lines are inserted.",
            default: "0",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "blank_lines_upper_bound",
            doc: "Maximum number of blank lines which can be put between items. If more than this number of consecutive empty lines are found, they are trimmed down to match this integer.",
            default: "1",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "brace_style",
            doc: "Brace style for items.",
            default: "\"SameLineWhere\"",
            values: vec!["\"AlwaysNextLine\"", "\"PreferSameLine\"", "\"SameLineWhere\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "chain_width",
            doc: "Maximum width of a chain to fit on one line.",
            default: "60",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "color",
            doc: "Whether to use colored output or not.",
            default: "\"Auto\"",
            values: vec!["\"Auto\"", "\"Always\"", "\"Never\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "combine_control_expr",
            doc: "Combine control expressions with function calls.",
            default: "true",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "comment_width",
            doc: "Maximum length of comments. No effect unless `wrap_comments = true`.",
            default: "80",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "condense_wildcard_suffixes",
            doc: "Replace strings of _ wildcards by a single .. in tuple patterns.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "control_brace_style",
            doc: "Brace style for control flow constructs.",
            default: "\"AlwaysSameLine\"",
            values: vec!["\"AlwaysNextLine\"", "\"AlwaysSameLine\"", "\"ClosingNextLine\""],
            kind: ValueKind::Enum,
        },
        KeyInfo {
            name: "disable_all_formatting",
            doc: "Don't reformat anything.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "edition",
            doc: "Specifies which edition is used by the parser.",
            default: "2015",
            values: vec!["2015", "2018", "2021", "2024"],
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
            name: "enum_discrim_align_threshold",
            doc: "The maximum length of enum variant having discriminant, that gets vertically aligned with others. Variants without discriminants would be ignored for the purpose of alignment.",
            default: "0",
            values: vec![],
            kind: ValueKind::Integer,
        },
        KeyInfo {
            name: "error_on_line_overflow",
            doc: "Error if Rustfmt is unable to get all lines within `max_width`, except for comments and string literals. If this happens, then it is a bug in Rustfmt. You might be able to work around the bug by refactoring your code to avoid long/complex expressions, usually by extracting a local variable or using a shorter name.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "error_on_unformatted",
            doc: "Error if unable to get comments or string literals within `max_width`, or they are left with trailing whitespaces.",
            default: "false",
            values: vec!["true", "false"],
            kind: ValueKind::Bool,
        },
        KeyInfo {
            name: "fn_args_layout",
            doc: "This option is deprecated and has been renamed to `fn_params_layout` to better communicate that it affects the layout of parameters in function signatures.",
            default: "\"Tall\"",
            values: vec!["\"Compressed\"", "\"Tall\"", "\"Vertical\""],
            kind: ValueKind::Enum,
        },
        // KeyInfo {
        //     name: "",
        //     doc: "",
        //     default: "",
        //     values: vec![],
        //     kind: ValueKind::,
        // },
    ]
}

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

            LineContext::Value { key, partial } => {
                let key_info = match keys.iter().find(|k| k.name == key) {
                    Some(k) => k,
                    None => return Ok(None),
                };

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

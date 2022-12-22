use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use rand::{thread_rng, Rng};
use syn::{parse::Parse, parse_macro_input, visit_mut::VisitMut, ExprTry, Stmt};

#[proc_macro]
pub fn try_(input: TokenStream) -> TokenStream {
    let Block { stmts } = parse_macro_input!(input as Block);
    let mut block = syn::Block {
        brace_token: Default::default(),
        stmts,
    };

    let id = thread_rng().gen_range(1000..=9999);
    let id = syn::Ident::new(&format!("try_polyfill_label_{id}"), Span::call_site());
    let id = syn::Lifetime {
        apostrophe: Span::call_site(),
        ident: id,
    };
    TryVisitor(id.clone()).visit_block_mut(&mut block);

    quote! {
        #id: {
            ::try_polyfill::Try::from_continue(#block)
        }
    }
    .into()
}

struct Block {
    stmts: Vec<Stmt>,
}

impl Parse for Block {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        syn::Block::parse_within(input).map(|stmts| Self { stmts })
    }
}

struct TryVisitor(syn::Lifetime);

macro_rules! path {
    ($span:expr => $(::$x:ident)+) => {
        syn::Path {
            leading_colon: Some(syn::token::Colon2($span)),
            segments: syn::punctuated::Punctuated::from_iter([$(
                syn::PathSegment {
                    ident: syn::Ident::new(stringify!($x), $span),
                    arguments: syn::PathArguments::None,
                },
            )*]),
        }
    };
}

impl VisitMut for TryVisitor {
    fn visit_expr_mut(&mut self, i: &mut syn::Expr) {
        match i {
            // turn try expressions into label-break-value
            syn::Expr::Try(ExprTry {
                expr,
                question_token,
                attrs,
            }) => {
                syn::visit_mut::visit_expr_mut(self, expr);
                *i = syn::Expr::Match(syn::ExprMatch {
                    attrs: std::mem::take(attrs),
                    match_token: syn::token::Match(question_token.span),
                    expr: Box::new(syn::Expr::Call(syn::ExprCall {
                        attrs: Vec::new(),
                        func: Box::new(syn::Expr::Path(syn::ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: path!(question_token.span => ::try_polyfill::__private::branch),
                        })),
                        paren_token: syn::token::Paren(question_token.span),
                        args: syn::punctuated::Punctuated::from_iter([std::mem::replace::<
                            syn::Expr,
                        >(
                            expr,
                            syn::Expr::Verbatim(Default::default()),
                        )]),
                    })),
                    brace_token: syn::token::Brace(question_token.span),
                    arms: vec![
                        syn::Arm {
                            attrs: Vec::new(),
                            pat: syn::Pat::TupleStruct(syn::PatTupleStruct {
                                attrs: Vec::new(),
                                path: path!(question_token.span => ::try_polyfill::__private::ControlFlow::Break),
                                pat: syn::PatTuple {
                                    attrs: Vec::new(),
                                    paren_token: syn::token::Paren(question_token.span),
                                    elems: syn::punctuated::Punctuated::from_iter([
                                        syn::Pat::Ident(syn::PatIdent {
                                            attrs: Vec::new(),
                                            by_ref: None,
                                            mutability: None,
                                            ident: syn::Ident::new("b", question_token.span),
                                            subpat: None,
                                        }),
                                    ]),
                                },
                            }),
                            guard: None,
                            fat_arrow_token: syn::token::FatArrow(question_token.span),
                            body: Box::new(syn::Expr::Break(syn::ExprBreak {
                                attrs: Vec::new(),
                                break_token: syn::token::Break(question_token.span),
                                label: Some(self.0.clone()),
                                expr: Some(Box::new(syn::Expr::Path(syn::ExprPath {
                                    attrs: Vec::new(),
                                    qself: None,
                                    path: syn::Ident::new("b", question_token.span).into(),
                                }))),
                            })),
                            comma: Some(syn::token::Comma(question_token.span)),
                        },
                        syn::Arm {
                            attrs: Vec::new(),
                            pat: syn::Pat::TupleStruct(syn::PatTupleStruct {
                                attrs: Vec::new(),
                                path: path!(question_token.span => ::try_polyfill::__private::ControlFlow::Continue),
                                pat: syn::PatTuple {
                                    attrs: Vec::new(),
                                    paren_token: syn::token::Paren(question_token.span),
                                    elems: syn::punctuated::Punctuated::from_iter([
                                        syn::Pat::Ident(syn::PatIdent {
                                            attrs: Vec::new(),
                                            by_ref: None,
                                            mutability: None,
                                            ident: syn::Ident::new("c", question_token.span),
                                            subpat: None,
                                        }),
                                    ]),
                                },
                            }),
                            guard: None,
                            fat_arrow_token: syn::token::FatArrow(question_token.span),
                            body: Box::new(syn::Expr::Path(syn::ExprPath {
                                attrs: Vec::new(),
                                qself: None,
                                path: syn::Ident::new("c", question_token.span).into(),
                            })),
                            comma: Some(syn::token::Comma(question_token.span)),
                        },
                    ],
                });

                // *i = parse_quote_spanned!(question_token.span =>
                //     match ::try_polyfill::__private::branch(#expr) {
                //         ::try_polyfill::__private::ControlFlow::Break(b) => break 'try_polyfill_label b,
                //         ::try_polyfill::__private::ControlFlow::Continue(c) => c,
                //     }
                // )
            }
            // ignore these any deeper, as they all should have their own try context
            syn::Expr::Async(_)
            | syn::Expr::Closure(_)
            | syn::Expr::Macro(_)
            | syn::Expr::TryBlock(_) => {}
            // continue deeper
            _ => syn::visit_mut::visit_expr_mut(self, i),
            // syn::Expr::Array(_) => todo!(),
            // syn::Expr::Assign(_) => todo!(),
            // syn::Expr::AssignOp(_) => todo!(),
            // syn::Expr::Await(_) => todo!(),
            // syn::Expr::Binary(_) => todo!(),
            // syn::Expr::Block(_) => todo!(),
            // syn::Expr::Box(_) => todo!(),
            // syn::Expr::Break(_) => todo!(),
            // syn::Expr::Call(_) => todo!(),
            // syn::Expr::Cast(_) => todo!(),
            // syn::Expr::Continue(_) => todo!(),
            // syn::Expr::Field(_) => todo!(),
            // syn::Expr::ForLoop(_) => todo!(),
            // syn::Expr::Group(_) => todo!(),
            // syn::Expr::If(_) => todo!(),
            // syn::Expr::Index(_) => todo!(),
            // syn::Expr::Let(_) => todo!(),
            // syn::Expr::Lit(_) => todo!(),
            // syn::Expr::Loop(_) => todo!(),
            // syn::Expr::Match(_) => todo!(),
            // syn::Expr::MethodCall(_) => todo!(),
            // syn::Expr::Paren(_) => todo!(),
            // syn::Expr::Path(_) => todo!(),
            // syn::Expr::Range(_) => todo!(),
            // syn::Expr::Reference(_) => todo!(),
            // syn::Expr::Repeat(_) => todo!(),
            // syn::Expr::Return(_) => todo!(),
            // syn::Expr::Struct(_) => todo!(),
            // syn::Expr::Tuple(_) => todo!(),
            // syn::Expr::Type(_) => todo!(),
            // syn::Expr::Unary(_) => todo!(),
            // syn::Expr::Unsafe(_) => todo!(),
            // syn::Expr::Verbatim(_) => todo!(),
            // syn::Expr::While(_) => todo!(),
            // syn::Expr::Yield(_) => todo!(),
        }
    }
}

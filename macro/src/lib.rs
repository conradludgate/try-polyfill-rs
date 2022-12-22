use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse, parse_macro_input, parse_quote_spanned, visit_mut::VisitMut, ExprTry, Stmt,
};

#[proc_macro]
pub fn try_(input: TokenStream) -> TokenStream {
    let Block { stmts } = parse_macro_input!(input as Block);
    let mut block = syn::Block {
        brace_token: Default::default(),
        stmts,
    };

    TryVisitor.visit_block_mut(&mut block);

    quote! {
        'try_polyfill: {
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

struct TryVisitor;

impl VisitMut for TryVisitor {
    fn visit_expr_mut(&mut self, i: &mut syn::Expr) {
        match i {
            // turn try expressions into label-break-value
            syn::Expr::Try(ExprTry {
                expr,
                question_token,
                ..
            }) => {
                *i = parse_quote_spanned!(question_token.span =>
                    match ::try_polyfill::__private::branch(#expr) {
                        ::try_polyfill::__private::ControlFlow::Break(b) => break 'try_polyfill b,
                        ::try_polyfill::__private::ControlFlow::Continue(c) => c,
                    }
                )
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

use syn::__private::quote::__private::TokenStream;
use syn::__private::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Result, TypePath};

pub enum Idx {
    TypePath(TypePath),
    U32(u32),
}

impl ToTokens for Idx {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Idx::TypePath(tp) => tp.to_tokens(tokens),
            Idx::U32(i) => i.to_tokens(tokens),
        }
    }
}

pub struct PatchesDef {
    pub item_effects: Vec<(Idx, syn::Ident)>,
    pub move_effects: Vec<(Idx, syn::Ident)>,
    pub special_processes: Vec<(Idx, syn::Ident)>,
    pub glue: Option<String>,
}

impl Parse for PatchesDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut item_effects = Vec::new();
        let mut move_effects = Vec::new();
        let mut special_processes = Vec::new();
        let mut glue = None;

        while !input.is_empty() {
            while input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
            if input.peek(syn::LitStr) {
                // The glue code. This must be the end.
                glue = Some(input.parse::<syn::LitStr>()?.value());
                break;
            }
            let name = input.parse::<syn::Ident>()?;
            if input.peek(syn::Token![:]) {
                input.parse::<syn::Token![:]>()?;
                let typ = input.parse::<syn::Ident>()?.to_string();
                match typ.as_str() {
                    "special_process" => {
                        if let Ok(tp) = input.parse::<TypePath>() {
                            special_processes.push((Idx::TypePath(tp), name));
                        } else {
                            let i = input.parse::<syn::LitInt>()?;
                            let j = i.base10_digits();
                            special_processes.push((Idx::U32(j.parse::<u32>().unwrap()), name));
                        }
                    }
                    "item_effect" => {
                        if let Ok(tp) = input.parse::<TypePath>() {
                            item_effects.push((Idx::TypePath(tp), name));
                        } else {
                            let i = input.parse::<syn::LitInt>()?;
                            let j = i.base10_digits();
                            item_effects.push((Idx::U32(j.parse::<u32>().unwrap()), name));
                        }
                    }
                    "move_effect" => {
                        if let Ok(tp) = input.parse::<TypePath>() {
                            move_effects.push((Idx::TypePath(tp), name));
                        } else {
                            let i = input.parse::<syn::LitInt>()?;
                            let j = i.base10_digits();
                            move_effects.push((Idx::U32(j.parse::<u32>().unwrap()), name));
                        }
                    }
                    x => {
                        return Err(syn::Error::new(
                            input.span(),
                            format!("Unknown patch type for patch {name}: {x}"),
                        ));
                    }
                }
            }
        }
        Ok(Self {
            item_effects,
            move_effects,
            special_processes,
            glue,
        })
    }
}

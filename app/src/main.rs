#![no_std]
#![no_main]

extern crate panic_halt;
extern crate cortex_m_rt_macros as macros;

#[cfg(feature = "device")]
#[doc(inline)]
pub use macros::interrupt;
#[doc(inline)]
pub use macros::{entry, exception, pre_init};


// extern crate proc_macro;

// use proc_macro::TokenStream;
// use proc_macro2::Span;
// use quote::quote;
// use std::collections::HashSet;
// use std::iter;
// use syn::{
//     parse, parse_macro_input, spanned::Spanned, AttrStyle, Attribute, FnArg, Ident, Item, ItemFn,
//     ItemStatic, ReturnType, Stmt, Type, Visibility,
// };


// #[proc_macro_attribute]
// pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
//     let mut f = parse_macro_input!(input as ItemFn);

//     // check the function signature
//     let valid_signature = f.sig.constness.is_none()
//         && f.vis == Visibility::Inherited
//         && f.sig.abi.is_none()
//         && f.sig.inputs.is_empty()
//         && f.sig.generics.params.is_empty()
//         && f.sig.generics.where_clause.is_none()
//         && f.sig.variadic.is_none()
//         && match f.sig.output {
//             ReturnType::Default => false,
//             ReturnType::Type(_, ref ty) => match **ty {
//                 Type::Never(_) => true,
//                 _ => false,
//             },
//         };

//     if !valid_signature {
//         return parse::Error::new(
//             f.span(),
//             "`#[entry]` function must have signature `[unsafe] fn() -> !`",
//         )
//         .to_compile_error()
//         .into();
//     }

//     if !args.is_empty() {
//         return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
//             .to_compile_error()
//             .into();
//     }

//     // XXX should we blacklist other attributes?
//     let (statics, stmts) = match extract_static_muts(f.block.stmts) {
//         Err(e) => return e.to_compile_error().into(),
//         Ok(x) => x,
//     };

//     f.sig.ident = Ident::new(&format!("__cortex_m_rt_{}", f.sig.ident), Span::call_site());
//     f.sig.inputs.extend(statics.iter().map(|statik| {
//         let ident = &statik.ident;
//         let ty = &statik.ty;
//         let attrs = &statik.attrs;

//         // Note that we use an explicit `'static` lifetime for the entry point arguments. This makes
//         // it more flexible, and is sound here, since the entry will not be called again, ever.
//         syn::parse::<FnArg>(
//             quote!(#[allow(non_snake_case)] #(#attrs)* #ident: &'static mut #ty).into(),
//         )
//         .unwrap()
//     }));
//     f.block.stmts = stmts;

//     let tramp_ident = Ident::new(&format!("{}_trampoline", f.sig.ident), Span::call_site());
//     let ident = &f.sig.ident;

//     let resource_args = statics
//         .iter()
//         .map(|statik| {
//             let (ref cfgs, ref attrs) = extract_cfgs(statik.attrs.clone());
//             let ident = &statik.ident;
//             let ty = &statik.ty;
//             let expr = &statik.expr;
//             quote! {
//                 #(#cfgs)*
//                 {
//                     #(#attrs)*
//                     static mut #ident: #ty = #expr;
//                     &mut #ident
//                 }
//             }
//         })
//         .collect::<Vec<_>>();

//     quote!(
//         #[doc(hidden)]
//         #[export_name = "main"]
//         pub unsafe extern "C" fn #tramp_ident() {
//             #ident(
//                 #(#resource_args),*
//             )
//         }

//         #f
//     )
//     .into()
// }

#[entry]
fn main() -> ! {
    loop {

    }
}


#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultHandler_() -> ! {
    // const SCB_ICSR: *const u32 = 0xE000_ED04 as *const u32;

    // let irqn = core::ptr::read(SCB_ICSR) as u8 as i16 - 16;

    panic!("DefaultHandler");
}


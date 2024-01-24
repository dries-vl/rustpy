extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn measure_time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let visibility = &input.vis;
    let attrs = &input.attrs;
    let sig = &input.sig;

    let expanded = quote! {
        #(#attrs)*
        #visibility #sig {
            let start = std::time::Instant::now();
            let result = (|| #fn_block)();
            let duration = start.elapsed();
            let seconds = duration.as_secs();
            let millis = duration.as_millis();
            let micros = duration.as_micros();
            println!("Time taken by {}: {}s {}ms {}μs", stringify!(#fn_name), seconds, millis, micros);
            result
        }
    };

    TokenStream::from(expanded)
}

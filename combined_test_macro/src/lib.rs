// combined_test_macro/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// This attribute macro generates two test functions, one for Tokio runtime and another for WebAssembly.
///
/// It allows you to write a single test function that works with both the Tokio runtime and the
/// `wasm_bindgen_test` framework.
///
/// # Examples
///
/// ```
/// # use combined_test_macro::combined_test;
/// #[combined_test]
/// async fn my_test() {
///     // Your test code here...
/// }
/// ```
#[proc_macro_attribute]
pub fn combined_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;

    let gen = quote! {
        #[cfg(not(target_arch = "wasm32"))]
        #[tokio::test]
        async fn #fn_name() {
            #input
        }

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen_test::wasm_bindgen_test]
        async fn #fn_name() {
            #input
        }
    };

    gen.into()
}

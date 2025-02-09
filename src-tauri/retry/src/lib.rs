use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitInt};

#[proc_macro_attribute]
pub fn retry(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(input as ItemFn);
    let retry_count = parse_macro_input!(args as LitInt).base10_parse::<u32>().unwrap();

    let fn_block = &function.block;
    let is_async = function.sig.asyncness.is_some();

    let retry_logic = if is_async {
        quote! {
            let result = async { #fn_block }.await;
            match result {
                Ok(val) => return Ok(val),
                Err(err) => {
                    if retries >= #retry_count {
                        return Err(err);
                    }
                    retries += 1;
                    eprintln!("Retry {}/{}: Error: {:?}", retries, #retry_count, err);
                    delay *= 2;
                    tokio::time::sleep(tokio::time::Duration::from_secs(std::cmp::min(delay, 32))).await;
                    continue;
                }
            }
        }
    } else {
        quote! {
            let result = (|| #fn_block)();
            match result {
                Ok(val) => return Ok(val),
                Err(err) => {
                    if retries >= #retry_count {
                        return Err(err);
                    }
                    retries += 1;
                    eprintln!("Retry {}/{}: Error: {:?}", retries, #retry_count, err);
                    delay *= 2;
                    std::thread::sleep(std::time::Duration::from_secs(std::cmp::min(delay, 32)));
                    continue;
                }
            }
        }
    };

    let new_block = quote! {
        {
            let mut retries = 0;
            let mut delay = 1;

            loop {
                #retry_logic
            }
        }
    };

    function.block = syn::parse2(new_block).unwrap();

    TokenStream::from(quote! {
        #function
    })
}
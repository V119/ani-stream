extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitInt};

#[proc_macro_attribute]
pub fn retry(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析输入的函数
    let mut function = parse_macro_input!(input as ItemFn);

    // 解析重试次数参数
    let retry_count = parse_macro_input!(args as LitInt).base10_parse::<u32>().unwrap();

    // 获取函数的名称和块
    let fn_name = &function.sig.ident;
    let fn_block = &function.block;

    // 生成新的函数体
    let new_block = quote! {
        {
            let mut retries = 0;
            let mut delay = 1; // 初始延迟时间为1秒

            loop {
                let result = (|| #fn_block)();

                match result {
                    Ok(val) => return Ok(val),
                    Err(err) => {
                        if retries >= #retry_count {
                            return Err(err);
                        }
                        retries += 1;
                        eprintln!("Retry {}: Error: {:?}", retries, err);
                        std::thread::sleep(std::time::Duration::from_secs(delay));
                        delay *= 2; // 指数递增延迟时间
                    }
                }
            }
        }
    };

    // 替换原始函数体
    function.block = syn::parse2(new_block).unwrap();

    // 返回生成的函数
    TokenStream::from(quote! {
        #function
    })
}
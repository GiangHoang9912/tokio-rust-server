use proc_macro::TokenStream;
use quote::quote;
use syn::{
  parse::{Parse, ParseStream},
  parse_macro_input, Expr, Result, Token,
};

struct SetupArgs {
  port: Expr,
  num_cpus: Expr,
}

impl Parse for SetupArgs {
  fn parse(input: ParseStream) -> Result<Self> {
    let port = input.parse()?;
    input.parse::<Token![,]>()?;
    let num_cpus = input.parse()?;
    Ok(SetupArgs { port, num_cpus })
  }
}

#[proc_macro]
pub fn setup(input: TokenStream) -> TokenStream {
  let SetupArgs { port, num_cpus } = parse_macro_input!(input as SetupArgs);

  let expanded = quote! {
    {
      println!("Server is running on:  http://0.0.0.0:{} on {} cpus", #port, #num_cpus);
    }
  };

  TokenStream::from(expanded)
}


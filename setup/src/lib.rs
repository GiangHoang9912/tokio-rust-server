use proc_macro::TokenStream;
use quote::quote;
use syn::{
  parse::{Parse, ParseStream},
  parse_macro_input, Expr, Result, Token,
};

struct SetupArgs {
  port: Expr,
  num_cpus: Expr,
  host: Expr,
}

impl Parse for SetupArgs {
  fn parse(input: ParseStream) -> Result<Self> {
    let port = input.parse()?;
    input.parse::<Token![,]>()?;
    let num_cpus = input.parse()?;
    input.parse::<Token![,]>()?;
    let host = input.parse()?;
    Ok(SetupArgs { port, num_cpus, host })
  }
}

#[proc_macro]
pub fn setup(input: TokenStream) -> TokenStream {
  let SetupArgs { port, num_cpus, host } = parse_macro_input!(input as SetupArgs);

  let expanded = quote! {
    {
      info!("Server is running on:  http://{}:{} on {} cpus",#host, #port, #num_cpus );
    }
  };

  TokenStream::from(expanded)
}

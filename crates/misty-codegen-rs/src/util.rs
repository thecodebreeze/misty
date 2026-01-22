use blake3::Hash;
use proc_macro2::TokenStream;
use quote::quote;

/// Helper function to generate a discriminator const from a hash.
pub fn generate_discriminator(hash: Hash) -> TokenStream {
    let bytes = hash.as_bytes().iter();
    quote! {
        &[#(#bytes), *]
    }
}

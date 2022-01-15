use crate::test_case::ast::TestCaseAST;

use ::proc_macro2::TokenStream;
use ::quote::format_ident;
use ::quote::quote;

pub fn build(ast: TestCaseAST, test_description_prefix: &'static str) -> TokenStream {
    let test_description = &ast.test_description;
    let test_description_ident = format_ident!("{test_description_prefix}_{test_description}");
    let test_name_ident = format_ident!("{}", &ast.test_name);

    quote! {
        #[test]
        fn #test_description_ident() {
          #test_name_ident()
        }
    }
}

#[cfg(test)]
mod build {
    use super::*;
    use ::pretty_assertions::assert_eq;

    #[test]
    fn it_should_output_test_function_with_wrapper() {
        let output = build(
            TestCaseAST {
                test_description: "should_do_blah".to_string(),
                test_name: "my_test_function".to_string(),
            },
            &"it",
        );

        let expected = quote! {
          #[test]
          fn it_should_do_blah() {
            my_test_function()
          }
        };

        assert_eq!(output.to_string(), expected.to_string());
    }
}

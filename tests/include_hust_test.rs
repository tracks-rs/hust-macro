use hust_macro::include_hust;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_include_view_with_index_hust() {
        let variable_name = "Variable Included!";
        
        let result = include_hust!("test.hust");
        let result_strip_whitespace = result.to_string().replace(" ", "").replace("\n", "").replace("\r", "");

        println!("{}",result_strip_whitespace);
        assert!(result_strip_whitespace.to_string().contains("<h1>Hello,World!</h1><p>HellofromHust!</p>VariableIncluded!"));
    }
}

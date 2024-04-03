pub fn add_commas(verify_code: String) -> String {
    let x: Vec<String> = verify_code.chars().map(|c| c.to_string()).collect();
    x.join(",")
}

#[cfg(test)]
mod tests {
    use super::add_commas;

    #[test]
    fn test_add_dashes() {
        let expected = "1,2,3,4,5".to_string();

        assert_eq!(expected, add_commas("12345".into()));
    }
}

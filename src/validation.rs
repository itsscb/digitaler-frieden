use regex::Regex;

pub fn validate_email(email: &str) -> bool {
    // Regular expression pattern for email validation
    let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();

    email_regex.is_match(email)
}

pub fn validate_phone_number(phone_number: &str) -> bool {
    // Regular expression pattern for phone number validation (all countries)
    let phone_regex = Regex::new(r"^[\+0][\-.,\(\)\d\s]*$").unwrap();

    phone_regex.is_match(phone_number)
}

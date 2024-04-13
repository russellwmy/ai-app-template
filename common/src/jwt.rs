use jwt::{Error, Header, RegisteredClaims, Token};

pub fn extract_sub_from_jwt(token: &str) -> Result<String, Error> {
    let unverified: Token<Header, RegisteredClaims, _> = Token::parse_unverified(token)?;
    let sub = unverified.claims().subject.clone();
    Ok(sub.unwrap())
}

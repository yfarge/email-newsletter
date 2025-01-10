#[derive(Debug)]
pub struct SubscriptionToken(String);

impl SubscriptionToken {
    pub fn parse(s: String) -> Result<SubscriptionToken, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.chars().count() > 25;
        let is_too_short = s.chars().count() < 25;
        let contains_forbidden_characters = s.chars().any(|c| !c.is_alphanumeric());

        if is_empty_or_whitespace || is_too_long || is_too_short || contains_forbidden_characters {
            Err(format!("{} is not a valid subscription token.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriptionToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriptionToken;
    use claims::assert_err;
    use rand::{distributions::Alphanumeric, rngs::StdRng, Rng, SeedableRng};

    #[test]
    fn empty_string_is_rejected() {
        let token = "".to_string();
        assert_err!(SubscriptionToken::parse(token));
    }

    #[test]
    fn a_token_longer_than_25_alphanums_is_rejected() {
        let token = "a".repeat(26);
        assert_err!(SubscriptionToken::parse(token));
    }

    #[test]
    fn a_token_shorter_than_25_alphanums_is_rejected() {
        let token = "a".repeat(24);
        assert_err!(SubscriptionToken::parse(token));
    }

    #[derive(Debug, Clone)]
    struct InvalidTokenFixture(pub String);

    impl quickcheck::Arbitrary for InvalidTokenFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let non_alphanumeric_chars = r"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));

            let token: String = (0..25)
                .map(|_| {
                    let idx = rng.gen_range(0..non_alphanumeric_chars.len());
                    non_alphanumeric_chars.chars().nth(idx).unwrap()
                })
                .collect();

            Self(token)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn invalid_tokens_are_rejected(invalid_token: InvalidTokenFixture) -> bool {
        SubscriptionToken::parse(invalid_token.0).is_err()
    }

    #[derive(Debug, Clone)]
    struct ValidTokenFixture(pub String);

    impl quickcheck::Arbitrary for ValidTokenFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let token = std::iter::repeat_with(|| rng.sample(Alphanumeric))
                .map(char::from)
                .take(25)
                .collect();
            Self(token)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_tokens_are_parsed_successfully(valid_token: ValidTokenFixture) -> bool {
        SubscriptionToken::parse(valid_token.0).is_ok()
    }
}

use validator::ValidateLength;

/// An incoming call request that needs to be processed.
pub struct NewCallRequest {
    pub phone_number: CallRequestPhoneNumber,
    pub contact_name: CallRequestContactName,
}

#[derive(Debug)]
pub struct CallRequestContactName(String);
#[derive(Debug)]
pub struct CallRequestPhoneNumber(String);

impl AsRef<str> for CallRequestPhoneNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CallRequestContactName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl CallRequestContactName {
    pub fn parse(s: String) -> Result<CallRequestContactName, String> {
        if s.validate_length(Some(2), Some(128), None) {
            Ok(Self(s))
        } else {
            Err(format!("Invalid contact name: {}", s))
        }
    }
}

impl CallRequestPhoneNumber {
    pub fn parse(s: String) -> Result<CallRequestPhoneNumber, String> {
        if s.validate_length(Some(10), Some(12), None) {
            Ok(Self(s))
        } else {
            Err(format!("Invalid phone number: {}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CallRequestPhoneNumber;
    use claims::{assert_err, assert_ok};

    #[test]
    fn empty_phone_number_is_rejected() {
        let phone_number = "".to_string();
        assert_err!(CallRequestPhoneNumber::parse(phone_number));
    }

    #[test]
    fn phone_number_accepted_when_valid() {
        let phone_number = "3208946581".to_string();
        assert_ok!(CallRequestPhoneNumber::parse(phone_number));
    }
}

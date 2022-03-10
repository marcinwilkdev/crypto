pub mod ceasar;
pub mod substitution;

fn check_message_from_letters(message: &[u8]) {
    for letter in message {
        assert!(*letter >= 97 && *letter <= 122, "invalid message");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_from_letters() {
        let message = b"hello";
        check_message_from_letters(message);
    }

    #[test]
    #[should_panic]
    fn message_not_from_letters() {
        let message = [97, 98, 53];
        check_message_from_letters(&message);
    }
}

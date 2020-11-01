/* The protocol has two commands:

1. PUBLISH <message>\n

2. RETRIEVE\n

With the additional properties:

1. Messages cannot contain newlines.

2. A missing newline at the end of the message is an error.

3. Data after the first newline is an error.

4. Empty messages are allowed. In this case, the message is PUBLISH \n.

*/

#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Publish(String),
    Retrieve,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    UnknownVerb,
    UnexpectedPayload,
    MissingPayload,
    EmptyMessage,
    IncompleteMessage,
}

fn check_for_newline(message: &str) -> bool {
    message.chars().last().unwrap().is_whitespace()
}

pub fn parse(input: &str) -> Result<Command, Error> {
    let tok_input = input.splitn(2, ' ').collect::<Vec<_>>();

    let verb = tok_input.get(0);
    let message = tok_input.get(1);

    if verb.is_none() {
        return Err(Error::EmptyMessage);
    }

    println!("{:?}, {:?}", verb, message);

    match verb.unwrap().trim() {
        "RETRIEVE" => unimplemented!(),
        "PUBLISH" => {
            if check_for_newline(message.unwrap()) {
                println!("Message: {}", message.unwrap().trim().to_string());
                Ok(Command::Publish(message.unwrap().trim().to_string()))
            } else {
                Err(Error::UnexpectedPayload)
            }
        }
        "" => Err(Error::EmptyMessage),
        _ => Err(Error::UnknownVerb),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish() {
        let line = "PUBLISH TestMessage\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Publish("TestMessage".into()));
        assert_eq!(result, expected);
    }

    #[test]
    fn unexpected_payload_without_newline() {
        let line = "PUBLISH TestMessage";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_empty_message_when_none_value() {
        let line = "";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::EmptyMessage);
        assert_eq!(result, expected);
    }
}

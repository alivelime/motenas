use crate::model::parser::Parser;

pub trait Cache {
    fn get_parser_token<P: Parser>() -> Option<String>;
}

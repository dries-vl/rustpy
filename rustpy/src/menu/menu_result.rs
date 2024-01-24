use crate::button::button_result::ButtonResult;

#[derive(Debug)]
pub enum MenuResult {
    BAD,
    GOOD(ButtonResult)
}

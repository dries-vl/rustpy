use crate::button::button_result::ButtonResult;
use crate::menu::menu_result::MenuResult;

pub mod button_result;

/// For more information, see [ButtonResult][`crate::button_result::ButtonResult`].
/// Corresponds to the **BUTTON** table <br>
/// `some code` Do not modify it *except* to align with the table in the db
/// Why no syntax highlighting?
#[derive(Debug)]
pub struct Button {
    pub id: String,
    menu: MenuResult,
}

impl Clone for Button {
    fn clone(&self) -> Self {
        todo!()
    }
}

// todo: add fields
// todo: add db connection
// todo: fix pointer issue
impl Button {
    pub fn new(id: String) -> Self {
        Self { id, menu: MenuResult::GOOD(ButtonResult::BROKEN) }
    }

    pub fn do_something(&self) {
        println!("Doing something with button {}", self.id);
    }
}

use chrono::{DateTime, Utc};

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
struct Store {
    #[validate()]
    pub id: u32,
}

#[derive(Debug, Validate, Deserialize)]
pub struct List {
    pub id: u32,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub entries: Option<Vec<ListEntry>>,
}

enum State {
    UnChecked,
    Checked,
    NotAvailable,
    NotFound,
}
/**
* while retrieving items sort them by this field
* if entry was not found in store move it to the next sublist
*
*/
pub struct ListEntry {
    checked_time: DateTime<Utc>,
    state: State,
}

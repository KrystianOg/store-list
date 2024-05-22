use crate::schemas::lists::{List, ListEntry};

pub async fn find_all() -> Vec<List> {
    let l1 = List {
        id: 1,
        name: String::from("Something"),
        entries: None,
    };

    vec![l1]
}

pub async fn find_one(id: u32) -> List {
    let l1 = List {
        id,
        name: String::from("Something"),
        entries: None,
    };

    return l1;
}

pub async fn create(list: List) -> List {
    list
}

pub async fn update(list: List) -> List {
    list
}

pub async fn delete(list: List) -> List {
    list
}

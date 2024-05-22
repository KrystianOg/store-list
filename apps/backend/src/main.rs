use tide::prelude::*;
use tide::Result;

pub mod controllers;
pub mod schemas;
pub mod services;

#[async_std::main]
async fn main() -> Result<()> {
    let mut app = tide::new();

    let mut lists = app.at("/lists");
    lists
        .get(controllers::lists::find_all)
        .post(controllers::lists::create);

    lists
        .at("/:list_id")
        .get(controllers::lists::find_one)
        .delete(controllers::lists::delete)
        .put(controllers::lists::update);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

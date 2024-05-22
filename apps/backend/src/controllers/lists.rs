use tide::Request;

/**
* user created shopping lists
* TODO: add full text search for names
*/
pub async fn find_all(mut req: Request<()>) -> tide::Result<()> {
    Ok(())
}

/**
* display shopping list all info
*/
pub async fn find_one(mut req: Request<()>) -> tide::Result {
    let list = req.body_json().await?;
    Ok(())
}

/**
* create shopping list
*/
pub async fn create(mut req: Request<()>) -> tide::Result<()> {
    let list = req.body_json().await?;
    Ok(())
}

pub async fn update(mut req: Request<()>) -> tide::Result<()> {
    Ok(())
}
pub async fn delete(mut req: Request<()>) -> tide::Result<()> {
    Ok(())
}

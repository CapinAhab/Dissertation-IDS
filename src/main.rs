#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

use rocket::fs::{FileServer, relative};

//Sets up static file paths
mod manual {
    use std::path::{PathBuf, Path};
    use rocket::fs::NamedFile;

    #[rocket::get("/<path..>")]
    pub async fn file_path(path: PathBuf) -> Option<NamedFile> {
        let mut path = Path::new(super::relative!("static")).join(path);
        if path.is_dir() {
            path.push("pages/index.html");
        }

        NamedFile::open(path).await.ok()
    }
}


/*#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}*/


//No need to manually route pages as all satic files setup. might nedd to change if using templates in the future

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![manual::file_path])
        .mount("/", FileServer::from(relative!("static")))
}

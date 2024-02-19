#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

use rocket::fs::{FileServer, relative};

use rocket::fs::NamedFile;

mod packet_capture;

//Sets up static file paths
mod manual {
    use rocket::fs::NamedFile;
    use std::path::{PathBuf, Path};

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


#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/pages/index.html").await.ok()
}

#[get("/dataset")]
async fn dataset() -> Option<NamedFile> {
    NamedFile::open("static/pages/data.html").await.ok()
}

#[get("/train")]
async fn train() -> Option<NamedFile> {
    NamedFile::open("static/pages/train.html").await.ok()
}

#[get("/model-info")]
async fn model_info() -> Option<NamedFile> {
    NamedFile::open("static/pages/model.html").await.ok()
}

//Use launch rather than main for async functionality
#[launch]
fn rocket() -> _ {
    rocket::build()
	.mount("/", routes![index])
	.mount("/", routes![dataset])
	.mount("/", routes![train])
	.mount("/", routes![model-info])
        .mount("/", routes![manual::file_path])
        //.mount("/", FileServer::from(relative!("static")))
}

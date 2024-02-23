#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

use rocket::fs::{FileServer, NamedFile, relative};
use rocket::tokio::task::spawn_blocking;
use rocket::response::Debug;

mod monitor_network;

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

#[get("/modelinfo")]
async fn modelinfo() -> Option<NamedFile> {
    NamedFile::open("static/pages/model.html").await.ok()
}

/*
fn async packet_function() -> Result<(), Debug<task::JoinError>>{
    let result = task::spawn_blocking(move || {
	capture_packets();
    }).await?;

    Ok(result)
}
}
*/

//Use launch rather than main for async functionality
#[launch]
fn rocket() -> _ {
    monitor_network::capture_packets();
   
    rocket::build()
	.mount("/", routes![index])
	.mount("/", routes![dataset])
	.mount("/", routes![train])
	.mount("/", routes![modelinfo])
        .mount("/", routes![manual::file_path])
	.mount("/", FileServer::from(relative!("static")))

}

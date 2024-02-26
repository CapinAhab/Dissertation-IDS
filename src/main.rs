#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

use rocket::fs::{FileServer, NamedFile, relative};
//use rocket::tokio::task::spawn_blocking;
//use rocket::response::Debug;
use rocket::serde::json::Json;

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

#[post("/gettraffic")]
async fn gettraffic() -> Json<monitor_network::FrontEndPacketData>{
    let interface = monitor_network::NetworkHandler::new();
    //Use while let Ok() = interface.get_packets()
    match interface.get_one_packet_front_end(){
	Err(value) => {
	    return Json(monitor_network::create_error_packet(value.to_string()))
	}
	Ok(value) => {
	    return Json(value)
	}
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
    //monitor_network::capture_packets();
   
    rocket::build()
	.mount("/", routes![index])
	.mount("/", routes![gettraffic])
	.mount("/", routes![dataset])
	.mount("/", routes![train])
	.mount("/", routes![modelinfo])
        .mount("/", routes![manual::file_path])
	.mount("/", FileServer::from(relative!("static")))

}

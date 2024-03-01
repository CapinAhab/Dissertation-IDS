#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

use rocket::fs::{FileServer, NamedFile, relative};
//use rocket::tokio::task::spawn_blocking;
//use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket_ws::{WebSocket, Stream};
use rocket::tokio::time::{sleep, Duration};
use crate::rocket::futures::SinkExt;
use rocket_ws::Message;
use tokio::time::interval;

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

#[get("/hello")]
fn hello(ws: WebSocket) -> rocket_ws::Channel<'static> {
    println!("fuck");
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let interval = Duration::from_secs(1);
            loop {
                let message = Message::Text("Hello".to_string());
                if let Err(err) = stream.send(message).await {
                    eprintln!("Error sending message: {}", err);
                    break; // Exit the loop if there's an error
                }
                sleep(interval).await; // Wait for the specified interval
            }
            Ok(())
        })
    })
}

#[post("/streamtest")]
async fn streamtest() -> Json<monitor_network::FrontEndPacketData>{
    let interface = monitor_network::NetworkHandler::new();
    //Use while let Ok() = interface.get_packets()
    println!("stream");
    while true{
	match interface.get_packets(){
	    Err(value) => {
		return Json(monitor_network::create_error_packet(value.to_string()))
	    }
	    Ok(value) => {
		return Json(value)
	    }
	}
    }
    return Json(monitor_network::create_error_packet(String::from("ahh")))
}

#[get("/gettraffic")]
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


//Use launch rather than main for async functionality
#[launch]
fn rocket() -> _ {
    //monitor_network::capture_packets();
   
    rocket::build()
	.mount("/", routes![index])
	.mount("/", routes![gettraffic])
	.mount("/", routes![dataset])
	.mount("/", routes![train])
	.mount("/", routes![hello])
	.mount("/", routes![streamtest])
	.mount("/", routes![modelinfo])
        .mount("/", routes![manual::file_path])
	.mount("/", FileServer::from(relative!("static")))

}

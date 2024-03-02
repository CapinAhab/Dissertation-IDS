#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

use rocket::fs::{FileServer, NamedFile, relative};
use rocket_ws::WebSocket;
use rocket::tokio::time::{sleep, Duration};
use crate::rocket::futures::SinkExt;
use rocket_ws::Message;

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

//Returns packet json as string
//websocket spec can only handle bytes and strings
#[get("/gettraffic")]
fn gettraffic(ws: WebSocket) -> rocket_ws::Channel<'static> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
		let interface = monitor_network::NetworkHandler::new();
		let mut message;
		//Might return error, sends the error as a string or successful json as a string
		match interface.get_one_packet_front_end(){
		    Err(value) => {
			message = Message::Text(value.to_string());
		    }
		    Ok(value) => {
			message = Message::Text(serde_json::to_string(&value).expect("Failed to convert json"));
		    }
		}
                
                if let Err(err) = stream.send(message).await {
                    eprintln!("Error sending message: {}", err);
                    break; // Exit the loop (end connection) if there's an error
                }
                sleep(Duration::from_millis(1)).await; // Wait for millisecond before getting next packet
            }
            Ok(())
        })
    })
}

//Dashboard
#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/pages/index.html").await.ok()
}

//Information on dataset
#[get("/dataset")]
async fn dataset() -> Option<NamedFile> {
    NamedFile::open("static/pages/data.html").await.ok()
}


//Options to train and tweak models
#[get("/train")]
async fn train() -> Option<NamedFile> {
    NamedFile::open("static/pages/train.html").await.ok()
}

//Stats on current model
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
	.mount("/", routes![modelinfo])
        .mount("/", routes![manual::file_path])
	.mount("/", FileServer::from(relative!("static")))

}

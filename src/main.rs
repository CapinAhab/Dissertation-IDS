#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

use rocket::fs::{FileServer, NamedFile, relative};
use rocket_ws::WebSocket;
use rocket::tokio::time::{sleep, Duration};
use crate::rocket::futures::SinkExt;
use rocket_ws::Message;
use rocket::form::Form;
use std::fs;

mod monitor_network;
mod deep_learn;


//Form input for model parameters
#[derive(Debug, FromForm)]
struct ModelPerameters{
    lstm_model: bool,
    layers: i64,
    neurons: i64
}


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
async fn gettraffic(ws: WebSocket) -> rocket_ws::Channel<'static> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
		let interface = monitor_network::NetworkHandler::new();
		let message;
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
    NamedFile::open("pages/index.html").await.ok()
}

//Information on dataset
#[get("/dataset")]
async fn dataset() -> Option<NamedFile> {
    NamedFile::open("pages/data.html").await.ok()
}


//Options to train and tweak models
#[get("/train")]
async fn train() -> Option<NamedFile> {
    //Only show train page if training file installed
    if fs::metadata("dataset/.gitignore").is_ok() {
	NamedFile::open("pages/train.html").await.ok()
    }
    else{
	NamedFile::open("pages/nomodel.html").await.ok()
    }
}

//Stats on current model
#[get("/modelinfo")]
async fn modelinfo() -> Option<NamedFile> {
    NamedFile::open("pages/model.html").await.ok()
}

#[post("/genmodel", data = "<model_data>")]
async fn genmodel(model_data: Form<ModelPerameters>){
    deep_learn::gen_net(model_data.layers, model_data.neurons, model_data.lstm_model);
}

//Use launch rather than main for async functionality
#[launch]
fn rocket() -> _ {
    rocket::build()
	.mount("/", routes![index])
	.mount("/", routes![gettraffic])
	.mount("/", routes![dataset])
	.mount("/", routes![train])
	.mount("/", routes![genmodel])
	.mount("/", routes![modelinfo])
        .mount("/", routes![manual::file_path])
	.mount("/", FileServer::from(relative!("static")))

}

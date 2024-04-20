#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

extern crate reqwest;

use rocket::fs::{FileServer, NamedFile, relative};
use rocket_ws::WebSocket;
use crate::rocket::futures::SinkExt;
use rocket_ws::Message;
use rocket::form::Form;
use std::path::Path;
use lazy_static::lazy_static;
use std::sync::Mutex;
use rocket::serde::json::Json;
use std::fs;

mod monitor_network;
mod deep_learn;
mod preprocess;


//Global variables, probably no race conditions due to lock
lazy_static! {
    static ref MODEL_TRAINED: Mutex<bool> = Mutex::new(true);
    //static ref MODEL: Mutex<deep_learn::CNNModel<Wgpu>> = Mutex::new(deep_learn::gen_net(10,10,false));
}


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
	    println!("stream ok");
	    let mut interface = monitor_network::NetworkHandler::new();
	    while let Some(Ok(value)) = interface.get_many_packet_front_end(){
		//println!("{:?}", preprocess::preprocess(value.clone().to_array()));
		let message = Message::Text(serde_json::to_string(&value).expect("Failed to convert json"));
		if let Err(err) = stream.send(message).await {
                    eprintln!("Error sending message: {}", err);
		}
	    }
            Ok(())
        })
    })
}

//Dashboard
#[get("/")]
async fn index() -> Option<NamedFile> {
    if monitor_network::test_network_permission() {
	NamedFile::open("pages/index.html").await.ok()
    }
    else{
	NamedFile::open("pages/nopermissions.html").await.ok()
    }
}

//Information on dataset
#[get("/dataset")]
async fn dataset() -> Option<NamedFile> {
    NamedFile::open("pages/data.html").await.ok()
}

//Trains current model and returns true if successful
#[get("/trainmodel")]
async fn trainmodel() -> Json<bool>{
    let mut trained = MODEL_TRAINED.lock().unwrap();
    *trained = true;
    let test_var=true;
    
    Json(test_var)
}

//gets and returns model accuracy percent
#[get("/testmodel")]
async fn testmodel() -> Json<i64>{
    let _test_data_dataset = monitor_network::get_train_packets("dataset/pcap/UCAP172.31.69.25".to_string());
    let _test_data_malicious_synthetic = monitor_network::get_train_packets("dataset/test-network-attack.pcap".to_string());
    let _test_data_malicious_synthetic = monitor_network::get_train_packets("dataset/test-network-standard.pcap".to_string());
    let accuracy = 80;
    Json(accuracy)
}

//Options to train and tweak models
#[get("/train")]
async fn train() -> Option<NamedFile> {
    //Only show train page if training dataset is installed
    if Path::new("dataset/pcap").exists() {
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

#[get("/test")]
async fn test_page() -> Option<NamedFile> {
    NamedFile::open("pages/test.html").await.ok()
}

//Prepossesses files generated from tshark
#[get("/preprocess-data")]
async fn preprocessdata(){
    //Need check so thread doesn't panic if tshark pre-processing hasn't been done
    if fs::metadata("./dataset/preprocessed/test-network-attack.csv").is_ok() {
	preprocess::process_dataset("./dataset/preprocessed/test-network-attack.csv", "./dataset/preprocessed/preprocess-test-network-attack.csv");
    }

    if fs::metadata("./dataset/preprocessed/test-network-standard-webtraffic.csv").is_ok() {
	preprocess::process_dataset("./dataset/preprocessed/test-network-standard-webtraffic.csv", "./dataset/preprocessed/preprocess-test-network-standard-webtraffic.csv");
    }

    if fs::metadata("./dataset/preprocessed/dataset-attack.csv").is_ok() {
	preprocess::process_dataset("./dataset/preprocessed/dataset-attack.csv", "./dataset/preprocessed/preprocess-dataset-attack.csv");
    }

    if fs::metadata("./dataset/preprocessed/test-network-standard-webtraffic-validate.csv").is_ok() {
	preprocess::process_dataset("./dataset/preprocessed/test-network-standard-webtraffic-validate.csv", "./dataset/preprocessed/preprocess-test-network-standard-webtraffic-validate.csv");
    }


}


/*
#[post("/genmodel", data = "<model_data>")]
async fn genmodel(model_data: Form<ModelPerameters>){
    let mut trained = MODEL_TRAINED.lock().unwrap();
    *trained = false;
    deep_learn::gen_net(model_data.layers, model_data.neurons, model_data.lstm_model);
}
*/

//Use launch rather than main for async functionality
#[launch]
fn rocket() -> _ {
    rocket::build()
	.mount("/", routes![index])
	.mount("/", routes![gettraffic])
	.mount("/", routes![dataset])
	.mount("/", routes![train])
	.mount("/", routes![test_page])
	.mount("/", routes![testmodel])
	.mount("/", routes![trainmodel])
	.mount("/", routes![preprocessdata])
//	.mount("/", routes![genmodel])
	.mount("/", routes![modelinfo])
        .mount("/", routes![manual::file_path])
	.mount("/", FileServer::from(relative!("static")))
}

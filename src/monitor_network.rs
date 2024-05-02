use etherparse::{SlicedPacket,TransportSlice};
use serde::{Serialize, Deserialize};
use ndarray::Array2;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::Packet;
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::datalink::Config;
use pnet::datalink::Channel;
use linfa_preprocessing::norm_scaling::NormScaler;
use linfa::prelude::*;


//probably the wrong way to do this
#[derive(Serialize, Deserialize)]
struct PacketJson {
    source_port: f64,
    destination_port: f64,
    sequence_number: f64,
    acknowledgment_number: f64,
    fin_flag: f64,
    syn_flag: f64,
    ack_flag: f64,
    psh_flag: f64,
    urg_flag: f64,
    window_size: f64,
    header_len: f64,
    tcp_len: f64
}


//struct reprsenting packet data, simplified for the front end
//inherits function that make it easy to convert to json
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrontEndPacketData{
    source: [u8; 6],
    destination: [u8; 6],
    protocole: u16,
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgment_number: u32,
    syn_flag: bool,
    ack_flag: bool,
    fin_flag: bool,
    rst_flag: bool,
    psh_flag: bool,
    urg_flag: bool,
    header_len: usize,
    window_size: u16,
    tcp_len: usize,
    malicious: bool
 
}

impl FrontEndPacketData {
    //Creates array of data to match dataset, not all data required for accurate prediction
    pub fn to_json(&self) -> String{
	//Cant  scale individual values, need to convert to array
        let mut array = Array2::from_shape_vec((1,12),
            vec![
                self.source_port as f64,
                self.destination_port as f64,
                self.sequence_number as f64,
                self.acknowledgment_number as f64,
		//Need to cast to u8 first or error
                self.fin_flag as u8 as f64,
                self.syn_flag as u8 as f64,
                self.ack_flag as u8 as f64,
                self.psh_flag as u8 as f64,
                self.urg_flag as u8 as f64,
		self.window_size as f64,
                self.header_len as f64,
		self.tcp_len as f64
            ]).expect("REASON");

	let scaler = NormScaler::l2();
	array = scaler.transform(array);

	let json_data = PacketJson {
	    source_port: array[[0,0]],
	    destination_port: array[[0,1]],
	    sequence_number: array[[0,2]],
	    acknowledgment_number: array[[0,3]],
	    fin_flag: array[[0,4]],
	    syn_flag: array[[0,5]],
	    ack_flag: array[[0,6]],
	    psh_flag: array[[0,7]],
	    urg_flag: array[[0,8]],
	    window_size: array[[0,9]],
	    header_len: array[[0,10]],
	    tcp_len: array[[0,11]], 
	};

	return serde_json::to_string(&json_data).expect("Failed to serialize JSON");
    }

    pub fn set_malicious(&mut self, value: bool){
	self.malicious=value;
    }
}

pub struct NetworkHandler {
    rx: Box<dyn datalink::DataLinkReceiver>,
}

impl NetworkHandler {
    pub fn new() -> Self {
        let interfaces = datalink::interfaces();
        
	let interface = interfaces.iter()
	    .find(|iface| iface.name == "wlp4s0")
	    .expect("Network interface 'wlp4s0' not found");

	let mut config = datalink::Config::default();
	    config.read_timeout = None;
	    config.promiscuous = true;

	let (tx, rx) = match datalink::channel(&interface, config) {
            Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!("Failed to create datalink channel: {}", e),
        };

        NetworkHandler { rx }
    }
    pub fn get_many_packet_front_end(&mut self) -> Option<Result<FrontEndPacketData, bool>> {
	match self.rx.next() {
	    Ok(packet) => {
		// Process the received packet
		let frame = EthernetPacket::new(packet).unwrap();
		match process_packet(frame.packet().to_vec()) {
		    Err(value) => return Some(Err(value)),
		    Ok(value) => return Some(Ok(value)),
		}
	    }
	    Err(_) => return None,
	}
    }
}

/*
pub fn get_train_packets(file_loc: String) -> Vec<FrontEndPacketData>{
    let mut cap = Capture::from_file(file_loc).unwrap();

    let mut captures = Vec::new();

    while let Ok(packet) = cap.next_packet() {
        println!("Packet captured: {:?}", packet);
	let frame = packet.to_vec();
	match process_packet(frame){
	    Ok(value) => {
		captures.push(value);
	    },
	    Err(_e) =>{
	    }
	}
    }

    return captures
}
*/
fn process_packet(frame :Vec<u8>) -> Result<FrontEndPacketData, bool>{ 
    match SlicedPacket::from_ethernet(&frame) {
	Err(value) => {
	    return Err(false)
	},
	Ok(value) => {
	    if let Some(transport) = value.clone().transport {
		match value.clone().transport.unwrap(){
		    TransportSlice::Tcp(tcp_slice) => {
			println!("{:?}", tcp_slice.clone());
			let packet = FrontEndPacketData{
			    source: value.link.clone().unwrap().to_header().unwrap().source,
			    destination: value.link.clone().unwrap().to_header().unwrap().destination,
			    protocole: value.link.unwrap().to_header().unwrap().ether_type.0,
			    source_port: tcp_slice.source_port(),
			    destination_port: tcp_slice.source_port(),
			    sequence_number: tcp_slice.sequence_number(),
			    acknowledgment_number: tcp_slice.acknowledgment_number(),
			    syn_flag: tcp_slice.syn(),
			    ack_flag: tcp_slice.ack(),
			    fin_flag: tcp_slice.fin(),
			    rst_flag: tcp_slice.rst(),
			    psh_flag: tcp_slice.psh(),
			    urg_flag: tcp_slice.urg(),
			    header_len: tcp_slice.header_len(),
			    window_size: tcp_slice.window_size(),
			    tcp_len: tcp_slice.payload().len(),
			    malicious: false

			};

			return Ok(packet)
		    },
		    _ => return Err(false)
		};
	    }
	    else{
		return Err(false);
	    }
	}
    }
}
//Iterates through available network devices and test for permission to access them
pub fn test_network_permission() -> bool {
    let interfaces = datalink::interfaces();

    // Choose the network interface you want to capture packets on
    let interface = interfaces.iter()
        .find(|iface| iface.is_up() && !iface.is_loopback())
        .expect("No usable network interface found");

    // Create a new packet capture handle for the selected interface
    let mut config = datalink::Config::default();
    config.read_timeout = None; // Set timeout duration to None for infinite timeout
   match datalink::channel(&interface, config) {
        Ok(Ethernet(_tx, _rx)) => return true,
        Ok(_) => return false,
        Err(_e) => return false,
    };
    

}

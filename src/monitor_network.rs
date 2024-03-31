use pcap::{Device, Capture};
use etherparse::{SlicedPacket,TransportSlice};
use etherparse::err::packet::SliceError;
use serde::{Serialize, Deserialize};

//struct reprsenting packet data, simplified for the front end
//inherits function that make it easy to convert to json
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
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
    header_len: usize
    
}

//Object that handles listening on the network
pub struct NetworkHandler{
    cap: Capture<pcap::Active>,
}


//Iterates through available network devices and test for permission to access them
pub fn test_network_permission() -> bool{
    match Device::lookup() {
        Ok(device) => {
            match device.expect("Error").open(){
		Ok(_) => return true,
		Err(_e) => return false
            }
	}
	Err(_e) => {
	    return false
	}
    }
}

impl NetworkHandler{
    //Constructor sets up background listener
    pub fn new() -> Self{
	let main_device = Device::lookup().unwrap().unwrap();
	NetworkHandler{
	    cap: Capture::from_device(main_device).unwrap()
		.promisc(true) //Needs to be in promiscuous mode to get all network traffic
		.snaplen(5000)
		.open().unwrap()
	}
    }

    //Gets packets and returns simplified struct to front end
    pub fn get_many_packet_front_end(&mut self) -> Option<Result<FrontEndPacketData, bool>>{
	while let Ok(packet) = self.cap.next_packet(){
	    let frame = packet.to_vec();
	    match process_packet(frame){
		Err(value) => {
		    return Some(Err(value))
		}
		Ok(value) => {
		    return Some(Ok(value))
		}
	    }
	}
	None
    }
}



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
			    header_len: tcp_slice.header_len()
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

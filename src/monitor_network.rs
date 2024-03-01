use pcap::{Device, Capture};
use etherparse::SlicedPacket;
use etherparse::err::packet::SliceError;
use libc;
use rocket::serde::Serialize;

//Struct to hold packet data, makes it easy to turn to jasona nd send to user
pub struct PacketData{
    //Using the time in the format provided by pcap
    ts: libc::timeval,
    caplen: u32,
    len: u32,
    link: i64,
    vlan: i64,
    net: i64,
    transport: i64
}

//struct reprsenting packet data
//inherits function that make it easy to convert to json
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FrontEndPacketData{
    source: [u8; 6],
    destination: [u8; 6],
    protocole: u16,
    error_message: String
}

pub struct NetworkHandler{
    cap: Capture<pcap::Active>,
}


impl NetworkHandler{
    pub fn new() -> Self{
	let main_device = Device::lookup().unwrap().unwrap();
	NetworkHandler{
	    cap: Capture::from_device(main_device).unwrap()
		.promisc(true)
		.snaplen(5000)
		.immediate_mode(true)
		.open().unwrap()
	}
    }
    
    pub fn get_packets(mut self) -> Result<FrontEndPacketData, SliceError>{
	while let Ok(packet) = self.cap.next_packet(){
	    let frame = packet.data.to_vec();
	    match Self::process_packet(frame){
		Err(value) => {
		    return Err(value)
		}
		Ok(value) => {
		    return Ok(value)
		}
	    }
	}
	return Ok(create_error_packet(String::from("No packets :(")))
    }

    //Gets packets and returns simplified struct to front end
    pub fn get_one_packet_front_end(mut self) -> Result<FrontEndPacketData, SliceError>{
	let packet = self.cap.next_packet();
	let frame = packet.unwrap().to_vec();
	match Self::process_packet(frame){
	    Err(value) => {
		return Err(value)
	    }
	    Ok(value) => {
		return Ok(value)
	    }
	}
    }

    //Converts packet into a struct
    fn process_packet(frame :Vec<u8>) -> Result<FrontEndPacketData, SliceError>{ 
	match SlicedPacket::from_ethernet(&frame) {
	    Err(value) => {
		return Err(value)
	    },
	    Ok(value) => {
		println!("link: {:?}", value.link);
		//println!("net: {:?}", value.net); 
		//println!("transport: {:?}", value.transport);
		let packet = FrontEndPacketData{
		    source: value.link.clone().unwrap().to_header().unwrap().source,
		    destination: value.link.clone().unwrap().to_header().unwrap().destination,
		    protocole: value.link.unwrap().to_header().unwrap().ether_type.0,
		    error_message: String::from("")
		};
		return Ok(packet)
	    }
	}
    }
}


//Creates packet with error message, kind of a work around
//Handled hear due to encapsulation on packet struck
pub fn create_error_packet(error_string: String) -> FrontEndPacketData{
	let error_packet = FrontEndPacketData{
	    source: [0,0,0,0,0,0],
	    destination: [0,0,0,0,0,0],
	    protocole: 0,
	    error_message: error_string
	};
    return error_packet
}

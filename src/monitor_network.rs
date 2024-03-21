use pcap::{Device, Capture};
use etherparse::SlicedPacket;
use etherparse::err::packet::SliceError;
use libc;
use serde::{Serialize, Deserialize};

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

//struct reprsenting packet data, simplified for the front end
//inherits function that make it easy to convert to json
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct FrontEndPacketData{
    source: [u8; 6],
    destination: [u8; 6],
    protocole: u16
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
		.immediate_mode(true)
		.open().unwrap()
	}
    }

    //Continuously gets packets
    //Code is for reference, using get_one_packet instead
    /*
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
     */

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

    //Converts packet into a struct for user front end
    fn process_packet(frame :Vec<u8>) -> Result<FrontEndPacketData, SliceError>{ 
	match SlicedPacket::from_ethernet(&frame) {
	    Err(value) => {
		return Err(value)
	    },
	    Ok(value) => {
		let packet = FrontEndPacketData{
		    source: value.link.clone().unwrap().to_header().unwrap().source,
		    destination: value.link.clone().unwrap().to_header().unwrap().destination,
		    protocole: value.link.unwrap().to_header().unwrap().ether_type.0,
		};
		return Ok(packet)
	    }
	}
    }
}



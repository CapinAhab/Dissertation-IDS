use pcap::{Device, Capture};
use etherparse::{SlicedPacket,TransportSlice};
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

    //Gets packets and returns simplified struct to front end
    pub fn get_one_packet_front_end(mut self) -> Result<FrontEndPacketData, bool>{
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
    fn process_packet(frame :Vec<u8>) -> Result<FrontEndPacketData, bool>{ 
	match SlicedPacket::from_ethernet(&frame) {
	    Err(value) => {
		return Err(false)
	    },
	    Ok(value) => {
		let test_var =value.clone().transport.unwrap();
		match test_var{
		    TransportSlice::Tcp(tcp_slice) => {

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
	}
    }
}



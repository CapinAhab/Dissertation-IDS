use pcap::{Device, Capture};
use etherparse::{SlicedPacket};
use libc;

//Struct to hold packet data, makes it easy to turn to jasona nd send to user
struct packet_data{
    //Using the time in the format provided by pcap
    ts: libc::timeval,
    caplen: u32,
    len: u32,
    link: i64,
    vlan: i64,
    net: i64,
    transport: i64
}


pub struct network_handler{
    cap: Capture<pcap::Active>,
}


impl network_handler{
    pub fn new() -> Self{
	let main_device = Device::lookup().unwrap().unwrap();
	network_handler{
	    cap: Capture::from_device(main_device).unwrap()
		.promisc(true)
		.snaplen(5000)
		.immediate_mode(true)
		.open().unwrap()
	}
    }


    pub fn get_packets(mut self){
	while let Ok(packet) = self.cap.next_packet(){
	    let frame = packet.data.to_vec();
	    Self::process_packet(frame)
	}
    }

    fn process_packet(frame :Vec<u8>) { 
	match SlicedPacket::from_ethernet(&frame) {
	    Err(value) => println!("Err {:?}", value),
	    Ok(value) => {
		println!("link: {:?}", value.link);
		println!("vlan: {:?}", value.vlan);
		println!("net: {:?}", value.net); // contains ip
		println!("transport: {:?}", value.transport);
	    }
	}
    }
}


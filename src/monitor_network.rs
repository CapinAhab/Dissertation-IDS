use pcap::{Device, Capture};
use etherparse::{SlicedPacket};

pub fn capture_packets() {
    let main_device = Device::lookup().unwrap().unwrap();
    let mut cap = Capture::from_device(main_device).unwrap()
	.promisc(true)
	.snaplen(5000)
	.immediate_mode(true)
	.open().unwrap();
    while let Ok(packet) = cap.next_packet() {
	let frame = packet.data.to_vec();
	process_packet(frame);
    }
}



fn process_packet(frame :Vec<u8>) { 
    match SlicedPacket::from_ethernet(&frame) {
	Err(value) => println!("Err {:?}", value),
	Ok(value) => {
	    println!("link: {:?}", value.link);
	    println!("vlan: {:?}", value.vlan);
	    //println!("net: {:?}", value.net); // contains ip
	    //println!("transport: {:?}", value.transport);
	}
    }
}

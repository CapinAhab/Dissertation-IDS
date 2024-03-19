//use tch::{nn, nn::OptimizerConfig, Device, Tensor};
extern crate tch;
use tch::{nn, nn::Module, nn::OptimizerConfig, Device, Tensor};


//Single layer LSTM may be effective, hinted at in lit
pub fn single_layer_lstm(){
    //number of input neurons
    let input_features = 35;

    //Gets hardware acceleration if possible, otherwise uses CPU
    let vs = nn::VarStore::new(Device::cuda_if_available());

    //LSTM network
    let input_layer = nn::lstm(vs.root(), 35, 10, Default::default());
}


pub fn gen_net(layers: i64, neurons: i64, lstm_model: bool){
    //number of input neurons
    let input_features = 35;

    //Gets hardware acceleration if possible, otherwise uses CPU
    let vs = nn::VarStore::new(Device::cuda_if_available());

    //Sequential network

    if lstm_model{
	println!("Not implemented yet");

    }
    else{
	//need a fixed number of input neurons to correspond to features
	let mut net = nn::seq();
	let input_layer = nn::linear(vs.root(), input_features, neurons, Default::default());
	net = net.add(input_layer);
	for _ in 0..layers{
	    net = net.add_fn(|xs| xs.relu());
	    net = net.add(nn::linear(vs.root(), neurons, neurons, Default::default()));
	}
	
	let output_layer = nn::linear(vs.root(), neurons, 10, Default::default());
	net = net.add_fn(|xs| xs.relu());
	net = net.add(output_layer);
	
    }
    
}

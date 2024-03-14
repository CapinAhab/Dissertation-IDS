//use tch::{nn, nn::OptimizerConfig, Device, Tensor};
extern crate tch;
use tch::{nn, nn::Module, nn::OptimizerConfig, Device, Tensor};

/*
pub struct LSTMnetwork {
    lstm: nn::LSTM,
    linear: nn::Linear,
}


impl nn::Module for LSTMnetwork{
    fn forward(&self, xs: &Tensor) -> Tensor {
        let output = xs.view([-1, 28]).apply(&self.lstm).view([-1, 128]);
        output.apply(&self.linear)
    }
}
*/
pub fn gen_net(layers: i64, neurons: i64, lstm_model: bool){
    //number of input neurons
    let input_features = 35;

    //Get cpu for machine learning
    let vs = nn::VarStore::new(Device::Cpu);

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

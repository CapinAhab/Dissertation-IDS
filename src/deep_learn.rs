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
pub fn test_net(){
    //Get cpu for machine learning
    let vs = nn::VarStore::new(Device::Cpu);
    let net = nn::seq()
        .add(nn::linear(vs.root(), 784, 256, Default::default()))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(vs.root(), 256, 10, Default::default()));
    
}

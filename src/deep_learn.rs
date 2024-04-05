use burn::{
    config::Config,
    module::Module,
    nn::{
        conv::{Conv2d, Conv2dConfig},
        conv::{Conv1d, Conv1dConfig},
        pool::{AdaptiveAvgPool2d, AdaptiveAvgPool2dConfig},
        Dropout, DropoutConfig, LstmConfig, Lstm, ReLU,
    },
    tensor::{backend::Backend, Tensor},
};
/*
pub struct DefaultModel<B: Backend> {
    input_layer:Lstm<B>,
    output_layer: Lstm<B>,
    activation: ReLU,
}

#[derive(Config)]
pub struct DefaultModelConfig {

    #[config(default = 35)]
    pub num_features: usize,

    #[config(default = 35)]
    pub hidden_size: usize,
}

impl DefaultModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> DefaultModel<B> {
        let input_layer = LstmConfig::new(self.num_features, self.hidden_size, true)
            .init(device);
        let output_layer = LstmConfig::new(self.hidden_size, 1, true)
            .init(device);

        DefaultModel {
            input_layer,
            output_layer,
            activation: ReLU::new(),
        }
    }
}
*/
/*
impl<B: Backend> DefaultModel<B> {
    pub fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 3> {
        let x = self.input_layer.forward(x);
        self.output_layer.forward(x)
    }
}
*/

pub struct CNNModel<B: Backend> {
    input_layer:Conv1d<B>,
    output_layer: Vec<Conv1d<B>>,
    activation: ReLU,
}

#[derive(Config)]
pub struct CNNModelConfig {

    #[config(default = 35)]
    pub num_features: usize,

    #[config(default = 35)]
    pub hidden_size: usize,

    #[config(default = 1)]
    pub layers: i64,
}

impl CNNModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> CNNModel<B> {
        let input_layer = Conv1dConfig::new(self.num_features, self.hidden_size, 3)
            .init(device);
        let mut output_layer = Vec::new();

	for _ in 1..self.layers{
	    output_layer.push(Conv1dConfig::new(self.hidden_size, 1, 3).init(device));
	}

        CNNModel{
            input_layer,
            output_layer,
            activation: ReLU::new(),
        }
    }
}

impl<B: Backend> CNNModel<B> {
    pub fn forward(&mut self, input: Tensor<B, 3>) -> Tensor<B, 3> {
	let mut x = input; //Dummy variable
        x = self.input_layer.forward(x);
	for layer in self.output_layer.iter_mut(){
	    x = self.activation.forward(x);
	    x = layer.forward(x);
	}
	return x
    }
}


pub fn gen_net(layers: i64, neurons: i64, lstm_model: bool){
    let model = false;
}

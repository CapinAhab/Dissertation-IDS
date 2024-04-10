use burn::{
    config::Config,
    module::Module,
    nn::{
        //conv::{Conv1d, Conv1dConfig},
        conv::{Conv1d, Conv1dConfig},
        ReLU,
    },
    tensor::{backend::Backend, Tensor},
};
use burn::backend::Wgpu;

//Default model, by deriving the model trait it describes a neural net
#[derive(Module, Debug)]
pub struct CNNModel<B: Backend> {
    input_layer:Conv1d<B>,
    output_layer: Vec<Conv1d<B>>,//Vec used to store variable number of layers
    activation: ReLU,
}

impl<B: Backend> CNNModel<B> {
    pub fn forward(&mut self, input: Tensor<B, 3>) -> Tensor<B, 3> {
	let mut x = input;
        x = self.input_layer.forward(x);
	for layer in self.output_layer.iter_mut(){
	    x = self.activation.forward(x);
	    x = layer.forward(x);
	}
	return x
    }
}


//Models can't be created directly, instead they have to be made via configs
//Config for CNN model, defaults to 1 layer of 35 input and output neurons
#[derive(Config,Debug)]
pub struct CNNModelConfig {

    #[config(default = 3)]
    pub num_features: i64,

    #[config(default = 35)]
    pub hidden_size: i64,

    #[config(default = 1)]
    pub layers: i64,
}

//Config lets user create numerous layers of the same size
//Returns configured model
impl CNNModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> CNNModel<B> {
        let input_layer = Conv1dConfig::new(self.num_features.try_into().unwrap(), self.hidden_size.try_into().unwrap(), 3)
            .init(device);

	//Vector holds variable number of layers
        let mut output_layer = Vec::new();

	for _ in 1..self.layers{
	    output_layer.push(Conv1dConfig::new(self.hidden_size.try_into().unwrap(), 1, 3).init(device));
	}

        CNNModel{
            input_layer,
            output_layer,
            activation: ReLU::new(),
        }
    }
}

pub fn gen_net(layers: i64, neurons: i64, lstm_model: bool) -> CNNModel<Wgpu>{
    let config = CNNModelConfig {
	num_features: 4,
	hidden_size: neurons,
	layers: layers,
    };

    let device = Default::default();
    let cnn_model: CNNModel<Wgpu> = config.init(&device);

    return cnn_model

}

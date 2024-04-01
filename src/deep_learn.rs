use burn::{
    config::Config,
    module::Module,
    nn::{
        conv::{Conv2d, Conv2dConfig},
        pool::{AdaptiveAvgPool2d, AdaptiveAvgPool2dConfig},
        Dropout, DropoutConfig, LstmConfig, Lstm, ReLU,
    },
    tensor::{backend::Backend, Tensor},
};

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
/*
impl<B: Backend> DefaultModel<B> {
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = input.detach();
        let x = self.input_layer.forward(x);
        let x = self.activation.forward(x);
        self.output_layer.forward(x)
    }
}
*/

pub fn gen_net(layers: i64, neurons: i64, lstm_model: bool){
    let model = false;
}

use burn::{
    data::{
        dataloader::batcher::Batcher,
        dataset::{
            transform::{PartialDataset, ShuffledDataset},
            Dataset, HuggingfaceDatasetLoader, SqliteDataset,
        },
    },
    prelude::*,
};

mod monitor_network;

#[derive(Clone, Debug)]
pub struct MNISTBatcher<B: Backend> {
    device: B::Device,
    packets: Tensor<B, 1>
    labels: Tensor<B, 1>
}

impl<B: Backend> MNISTBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}




pub fn generate_tensor_from_data(data: monitor_network::FrontEndPacketData, B: Backend) -> Tensor<B, 2>{
    let device = B::Device::default();
    //Convert all fields to u32
    //header_len might be cut short but unlikely to see in real world data
    let packet_tensor = Tensor::<B, 2>::from_ints(
	data.protocole as u32,
	data.source_port as u32,
	data.destination_port as u32,
	data.sequence_number,
	data.acknowledgment_number,
	data.syn_flag as u32,
	data.ack_flag as u32,
	data.fin_flag as u32,
	data.rst_flag as u32,
	data.psh_flag as u32,
	data.urg_flag as u32,
	data.header_len as u32
    );

    return packet_tensor
}

//All data divided by max datatype value to get them all on scale of 0-1
//Done so all features contribute equally (not thrown of by large values in packet length)
pub fn generate_tensor_from_data_min_max_scale(data: monitor_network::FrontEndPacketData, B: Backend) -> Tensor<B, 1>{
    let device = B::Device::default();
    //Convert all fields to u32
    let packet_tensor = Tensor::<B, 1>::from_floats(
	(data.protocole / u16::MAX) as f32,
	(data.source_port / u16::MAX) as f32,
	(data.destination_port / u16::MAX) as f32,
	(data.sequence_number / u32::MAX) as f32 ,
	(data.acknowledgment_number / u32) as f32,
	data.syn_flag as f32,
	data.ack_flag as f32,
	data.fin_flag as f32,
	data.rst_flag as f32,
	data.psh_flag as f32,
	data.urg_flag as f32,
	(data.header_len / u64::MAX) as f32
    );

    return packet_tensor
}

fn load_csv(file_path: &str) -> Result<Array2<f64>, ReadError>{
    let file = File::open(file_path).unwrap();
    return linfa_datasets::array_from_csv(file, true, b',')
}

fn preprocess(data: Array2<f64>){
    let dataset = Dataset::from(data);
    let embedding = Pca::params(1).fit(&dataset).unwrap();
    let dataset = embedding.transform(dataset);
    println!("{:?}", dataset.records());
}


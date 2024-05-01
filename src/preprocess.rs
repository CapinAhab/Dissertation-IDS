use std::fs::File;
use ndarray::Array2;
use ndarray_csv::ReadError;
use linfa::Dataset;
use linfa_reduction::Pca;
use linfa::prelude::*;
use csv::Writer;
use std::error::Error;
use ndarray::Axis;
use linfa_preprocessing::norm_scaling::NormScaler;


/*
pub fn generate_tensor_from_data(data: monitor_network::FrontEndPacketData, B: Backend) -> Tensor<B, 2>{
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
 */




//loads CSV as dataframe that can be preprocessed
fn load_csv(file_path: &str) -> Result<Array2<f64>, ReadError>{
    let file = File::open(file_path).unwrap();
    return linfa_datasets::array_from_csv(file, true, b',')
}

pub fn preprocess(data: Array2<f64>) -> Array2<f64>{
    let mut dataset = Dataset::from(data);
    //need to scale first or NaN errors in PCA
    let scaler = NormScaler::l2();
    dataset = scaler.transform(dataset);
    return dataset.records().to_owned();
}

//Applies PCA to reduce dimentionality
pub fn preprocess_pca(data: Array2<f64>) -> Array2<f64>{
    let mut dataset = Dataset::from(data);
    //need to scale first or NaN errors in PCA
    let scaler = NormScaler::l2();
    dataset = scaler.transform(dataset);
    let embedding = Pca::params(6).fit(&dataset).unwrap();
    dataset = embedding.transform(dataset);
    return dataset.records().to_owned();
}

fn save_to_csv(array: &Array2<f64>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;

    let mut writer = Writer::from_writer(file);
    let row_vectors: Vec<Vec<f64>> = array.axis_iter(Axis(0))
	.map(|row| row.to_vec())
        .collect();

    for row in row_vectors{
	writer.write_record(row.iter().map(|&x| x.to_string()))?;
    }


    writer.flush()?;

    Ok(())
}


pub fn process_dataset(file_path: &str, save_path: &str, pca: bool){
    match load_csv(file_path){
	Ok(dataset) => {
	    let preprocessed_dataset;
	    if pca{
		preprocessed_dataset = preprocess_pca(dataset);
	    }
	    else{
		preprocessed_dataset = preprocess(dataset);
	    }
	    match save_to_csv(&preprocessed_dataset, save_path){
		Ok(_done) => {
		    println!("Done");
		}
		Err(_e) => {
		    println!("Failed");
		}
	    }
	}
	Err(e) => {
	    println!("Error {:?}",e)
	}
    }
}


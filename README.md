## Dataset and custom models

The default model was trained with this dataset [https://www.unb.ca/cic/datasets/ids-2018.html](https://www.unb.ca/cic/datasets/ids-2018.html), it contains several attacks but the default this project is uses is focused on DOS attacks from the Slowloris tool. 

Users can create and train custom models but the dataset must be installed in the dataset directory for the feature to be enabled.

### Setup
To enable the feature you must download the dataset from its AWS bucket and place it in the dataset directory. bellow is an example of how to do this on debian.

```bash
sudo apt-get install awscli
aws s3 cp --no-sign-request --region <your-region> s3://cse-cic-ids2018/Original Network Traffic and Log data/Thursday-15-02-2018/pcap.zip .
unzip pcap.zip
mv pcap /path_to_project/dataset/
```

This can be done automatically through docker, the file Dockerfile-dataset. The command to use this Docker file isntead of the dfault is listed bellow, be aware this will take some time as the dataset is over 30GB big.

```bash
sudo docker build -t ids -f Dockerfile-dataset .
```

### Model validation

To ensure the models generated aren't overfitted to the training dataset, I have created testing data of malicious and non-malicious packet captures taken from a virtual network.

![Test network topology]("readme_images/virtual-network-topology.webp")

The above image shows the topology of the test network used: a virtual internal network (no internet access) with just a target Debian machine with an IP of 192.168.100.193 and an attacking Kali Linux machine with an IP of 192.168.100.193.

## Building

### From Source
The program is compiled with cargo like any other Rust project, however the version of the libuary tch is only compatable with libtorch version 2.2.0. The correct version of libtorch [https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.2.0%2Bcpu.zip](https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.2.0%2Bcpu.zip) but the compiler still needs to be pointed to it. By default tch will try to use a libuary at the location /usr/lib/libtorch.so, but the location of the libuary can be set with the environment variable LIBTORCH, bellow is an example of how to use it.

```bash
export LIBTORCH=/path_to_project/libtorch
cd path_to_project
cargo build
```

The compiler also uses the library libglog which has also been updated, so you will need to downgrade to version 0.6.0 or move the shared object files in the libglog directory to /usr/lib/.

```bash
mv libglog/ /usr/lib/
```

### Docker
Due to the complex build process, a docker file has been provided to give a consistent build environment where the correct dependencies can be installed automatically. Docker is significantly simpler than building from source and is the preferred method even with its higher resource use over bare metal. Below is an example of how to start Docker and then create and run the Docker image.

```bash
sudo systemctl start docker
cd path_to_project
sudo docker build -t ids .
sudo docker run --network host ids
```

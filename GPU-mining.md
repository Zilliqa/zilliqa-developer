# GPU mining

## Hardware requirement
The graphic card should have at least 2GB RAM.

## For OpenCL

If you wish to use OpenCL supported GPU for PoW. Please run `sudo apt install ocl-icd-opencl-dev` to install the OpenCL developer package. After which, use the following build option to build Zilliqa with OpenCL support.
```
./build.sh opencl
```
Before running Zilliqa application, please set the FULL_DATASET_MINE and OPENCL_GPU_MINE in constants.xml to true to enable the PoW using OpenCL GPU.
## For CUDA

If you wish to use CUDA supported GPU for PoW, please download and install CUDA package from [NVIDIA official webpage](https://developer.nvidia.com/cuda-downloads). You may need to reboot your PC for the installation to take effect. After which, use the following build option to build Zilliqa with CUDA support.
```
./build.sh cuda
```
Before starting Zilliqa application, please set the FULL_DATASET_MINE and CUDA_GPU_MINE in constants.xml to true to enable the PoW using CUDA GPU.
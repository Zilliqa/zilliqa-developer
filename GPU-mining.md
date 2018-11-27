# GPU mining

## Hardware requirement
The graphic card should have at least 2GB RAM.

## For OpenCL

If you wish to use OpenCL supported GPU for PoW. Please run `sudo apt install ocl-icd-opencl-dev` to install the OpenCL developer package. After which, use the following build option to build Zilliqa with OpenCL support.
```
./build.sh opencl
```
Before running Zilliqa application, please set the **OPENCL_GPU_MINE** in constants.xml to true to enable the PoW using OpenCL GPU.
## For CUDA

If you wish to use CUDA supported GPU for PoW, please download and install CUDA package from [NVIDIA official webpage](https://developer.nvidia.com/cuda-downloads). You may need to reboot your PC for the installation to take effect. After which, use the following build option to build Zilliqa with CUDA support.
```
./build.sh cuda
```
Before starting Zilliqa application, please set the **CUDA_GPU_MINE** in constants.xml to true to enable the PoW using CUDA GPU.
## For Multiple GPUs
If you have multiple OpenCL or CUDA GPUs, now they can work concurrently. Please edit the **GPU_TO_USE** in constants.xml to select the GPUs you want to use. The index start from 0, and you can use select one or multiple GPUs, for example, "0", "0, 1, 2", "0, 2, 4", but make sure the largest index within the physical number of GPUs in the PC. Then you can run Zilliqa with multiple GPUs for PoW.
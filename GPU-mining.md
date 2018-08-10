# GPU mining

## For OpenCL

The OpenCL support is built into the Zilliqa by default. If you have a graphics card with AMD GPU, you can use it for PoW already.

## For CUDA

If you wish to use CUDA supported GPU for PoW, please download and install CUDA package from [NVIDIA official webpage](https://developer.nvidia.com/cuda-downloads). You may need to reboot your PC for the installation take effect. After which, please use the following build option to build Zilliqa with CUDA support.

```
./build.sh cuda
```
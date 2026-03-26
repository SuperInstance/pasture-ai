# Install.sh Schema

## Purpose
High-level schema for an install script that detects hardware (e.g., NVIDIA RTX or Jetson) and sets appropriate variables.

## Variables
- **HAS_RTX**: Boolean - True if NVIDIA RTX GPU is detected.
- **IS_JETSON**: Boolean - True if running on NVIDIA Jetson hardware.
- **CUDA_VERSION**: String - Detected CUDA version if applicable.
- **INSTALL_PATH**: String - Default installation directory, adjustable based on hardware.

## Detection Logic Pseudocode
```
if command_exists(nvidia-smi):
    gpu_info = run(nvidia-smi --query-gpu=name --format=csv)
    if "RTX" in gpu_info:
        HAS_RTX = true
        CUDA_VERSION = run(nvidia-smi --query-gpu=driver_version --format=csv)  # Simplified

if file_exists(/etc/nv_tegra_release):
    IS_JETSON = true
    # Set Jetson-specific vars, e.g., JETPACK_VERSION from release file

if HAS_RTX or IS_JETSON:
    echo "Hardware detected. Setting environment..."
    export HAS_RTX
    export IS_JETSON
    # Proceed with hardware-optimized install
else:
    echo "No supported hardware detected. Falling back to CPU mode."
```

# 📦 The Arrival: Jetson Unboxing to First "Hello Cow"

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                              │
│                    📦 THE ARRIVAL - UNBOXING YOUR JETSON                    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## What You'll Need

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      SHOPPING LIST                                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  REQUIRED                                                                   │
│  ────────                                                                   │
│  □ NVIDIA Jetson Orin Nano 8GB Developer Kit                  ~$499        │
│  □ 64GB+ microSD card (128GB recommended)                     ~$20         │
│  □ 5V/4A power supply (included with dev kit)                 $0           │
│  □ USB-C cable for initial setup                              ~$10         │
│                                                                              │
│  RECOMMENDED                                                                │
│  ────────────                                                               │
│  □ Active cooling fan (Noctua A4x10)                         ~$15         │
│  □ Case (Geekworm or similar)                                ~$25         │
│  □ USB Wi-Fi dongle (if using dev kit)                       ~$15         │
│                                                                              │
│  OPTIONAL                                                                   │
│  ────────                                                                   │
│  □ GPIO sensors for Hogs                                     varies        │
│  □ NVMe SSD for faster I/O                                   ~$50         │
│                                                                              │
│  TOTAL: ~$500-600 (one-time investment)                                     │
│  Compare to: $2,400/year for cloud subscriptions                           │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Step 1: Flash the SD Card (10 minutes)

### Download the Image

```bash
# Download JetPack 6 (Ubuntu 22.04 + CUDA + TensorRT)
wget https://developer.nvidia.com/downloads/embedded/l4t/r36_release_v3.0/sd_card_bsp/jetson_orin_nano_8gb_sd_card.img

# Or use NVIDIA SDK Manager for more control
```

### Flash to SD Card

```bash
# Insert SD card, check device
lsblk | grep sd

# Flash (BE CAREFUL - replace sdX with your device!)
sudo dd if=jetson_orin_nano_8gb_sd_card.img of=/dev/sdX bs=4M status=progress && sync
```

## Step 2: First Boot (5 minutes)

1. Insert SD card into Jetson
2. Connect monitor (HDMI) and keyboard
3. Connect power
4. Follow the Ubuntu setup wizard
5. Create user: `rancher` / `superinstance`

## Step 3: Initial Configuration (5 minutes)

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install essentials
sudo apt install -y build-essential curl wget git cmake

# Enable maximum performance
sudo nvpmodel -m 0  # MAXN mode
sudo jetson_clocks   # Maximize clocks

# Create swap (CRITICAL for 8GB board)
sudo fallocate -l 16G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab

# Disable GUI for headless (saves ~1GB RAM)
sudo systemctl set-default multi-user.target
```

## Step 4: Install SuperInstance (5 minutes)

```bash
# One-command install
curl -sSL https://install.superinstance.ai | bash

# Or manual:
git clone https://github.com/SuperInstance/superinstance.git
cd superinstance
make install
```

## Step 5: First "Hello Cow" (1 minute)

```bash
# Start the Ranch
make run
```

**You should see:**

```
╔══════════════════════════════════════════════════════════════════════╗
║        SUPERINSTANCE RANCH - Day 1                                    ║
╠══════════════════════════════════════════════════════════════════════╣
║                                                                       ║
║  🐄 Cattle: Hello-Cow-v1 (loaded)                                    ║
║     └── VRAM: 2.5GB | Temp: 0.9 | Status: Ready                     ║
║                                                                       ║
║  Press [H] to say Hello to your first Cow!                           ║
╚══════════════════════════════════════════════════════════════════════╝
```

Press `H`:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                          │
│  🐄 Hello-Cow-v1 says:                                                  │
│                                                                          │
│  "Welcome to the Ranch! 🌅                                               │
│                                                                          │
│   I'm your first Cattle agent, ready to help you with:                  │
│   • Email triage and composition                                         │
│   • Document summarization                                               │
│   • Research and analysis                                                │
│                                                                          │
│   Edit my DNA at: pasture/cattle/hello-cow-v1/breed.md                  │
│                                                                          │
│   Night School starts at 02:00. I'll get smarter while you sleep!"      │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## 🎉 Congratulations!

You've just:

- ✅ Flashed your Jetson
- ✅ Optimized for AI workloads
- ✅ Created 16GB swap (critical!)
- ✅ Installed SuperInstance
- ✅ Met your first agent

Your total cloud savings start now. By this time next year, you'll have saved $2,400.

---

## Troubleshooting

### "Out of memory" errors

```bash
# Check swap is active
swapon --show

# Increase swap if needed
sudo swapoff /swapfile
sudo fallocate -l 24G /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

### Slow performance

```bash
# Check power mode
nvpmodel -q

# Should show: NV Power Mode: MAXN
# If not:
sudo nvpmodel -m 0
sudo jetson_clocks
```

### USB device not recognized

```bash
# Check device
lsusb

# Add to user groups
sudo usermod -aG dialout rancher
```

---

> *"The arrival is the hardest part. From here, it's all evolution."*

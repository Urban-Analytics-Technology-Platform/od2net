#!/bin/bash
# Ensure we are running under Bash
if [ -z "$BASH_VERSION" ]; then
    echo "This script requires Bash, but it was executed with $SHELL. Exiting."
    exit 1
fi

# Set debugging and exit on error
set -euxo pipefail

# Check the Linux distro we're running:
cat /etc/os-release

# Install Rust:
curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add cargo to the path both temporarily and permanently:
export PATH="$HOME/.cargo/bin:$PATH"
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.profile

# Ensure cargo command is available
command -v cargo

# Ensure apt repository is up-to-date and install necessary packages
sudo apt-get update
sudo apt-get install -y software-properties-common python3 python3-pip

# Install Python dependencies:
# Uncomment and modify if you have a requirements.txt
# pip3 install -r requirements.txt

# Clone and install tippecanoe if not already installed
cd /tmp
if [ ! -d "tippecanoe" ]; then
    git clone https://github.com/felt/tippecanoe.git
fi
cd tippecanoe
make -j$(nproc)
sudo make install
tippecanoe --version

# Install GitHub CLI
sudo apt install -y gh

# Make sure there's a newline at the end of the script
echo "Script execution completed successfully."

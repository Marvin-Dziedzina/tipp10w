#!/bin/bash

#
# This script builds the project for Linux and Windows and creates a tarball and a zip file.
#

# Update rust toolchain
echo "Updating toolchain..."
if ! rustup update
then
    echo "Toolchain update failed!"
    exit 1

    else
    echo "Toolchain update successful."
fi

# Build the linux version
echo "Building the Linux version..."
if ! sudo cargo build -r --target=x86_64-unknown-linux-gnu
then
    echo "Linux release build failed!"
    exit 1
    
    else
    echo "Linux release build successful."
fi

# Start docker
if ! systemctl is-active --quiet docker; then
    echo "Starting docker..."
    if ! sudo systemctl start docker
    then
        echo "Could not start docker!"
        exit 1

        else
        echo "Docker is started."
    fi
fi

# Build the windows version
echo "Building the Windows version..."
if ! cross build -r --target=x86_64-pc-windows-gnu
then
    echo "Windows release build failed!"
    exit 1
        
    else
    echo "Windows release build successful."
fi

echo "Stopping docker..."
if ! sudo systemctl stop docker
then
    echo "Docker stop failed!"

    else
    echo "Docker stopped."
fi

# Create the tarball
echo "Creating tarball..."
if ! sudo tar -czf tipp10w_linux_x86-x64.tar.gz -C target/x86_64-unknown-linux-gnu/release tipp10w
then
    echo "Tar compression failed!"
    exit 1

    else
    echo "Tar compression successful."
fi

# Create the zip file
echo "Creating zip file..."
if ! sudo zip -j tipp10w_windows_x86-64.zip target/x86_64-pc-windows-gnu/release/tipp10w.exe
then
    echo "Zip compression failed!"
    exit 1

    else
    echo "Zip compression successful."
fi

echo "Done!"
exit 0
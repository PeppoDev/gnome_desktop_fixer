#!/bin/bash

# Build the binary
echo "Building gnome-desktop-fixer..."
cargo build --release

# Install the binary
echo "Installing binary to /usr/local/bin/..."
sudo cp target/release/gnome_desktop_fixer /usr/local/bin/gnome-desktop-fixer
sudo chmod +x /usr/local/bin/gnome-desktop-fixer

# Create systemd user service
echo "Creating systemd user service..."

cat > /etc/systemd/system/gnome-desktop-fixer.service << EOF
[Unit]
Description=GNOME Desktop Fixer

[Service]
ExecStart=/usr/local/bin/gnome_desktop_fixer
Restart=always
User=$USER

[Install]
WantedBy=multi-user.target
EOF

# Enable and start the service
echo "Enabling and starting service..."
sudo systemctl daemon-reexec
sudo systemctl daemon-reload
sudo systemctl enable gnome-desktop-fixer.service

echo "Installation completed!"
echo "Service status: sudo systemctl status gnome-desktop-fixer.service" 
#!/bin/bash


SERVICE_NAME="gnome-desktop-fixer"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
BINARY_PATH="/usr/local/bin/${SERVICE_NAME}"

# Build the binary
echo "Building ${SERVICE_NAME}"
cargo build --release

# Install the binary
echo "Installing binary to ${BINARY_PATH}"
sudo cp target/release/gnome_desktop_fixer $BINARY_PATH
sudo chmod +x $BINARY_PATH

# Create systemd user service
echo "Creating systemd user service..."

cat > $SERVICE_FILE << EOF
[Unit]
Description=GNOME Desktop Fixer

[Service]
ExecStart=$BINARY_PATH --watch
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
#!/bin/bash

SERVICE_NAME="gnome-desktop-fixer"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
BINARY_PATH="/usr/local/bin/${SERVICE_NAME}"

# Check if service already exists
SERVICE_EXISTS=false
if systemctl list-unit-files | grep -q "^${SERVICE_NAME}.service"; then
    echo "Service ${SERVICE_NAME} already exists, skipping service creation."
    SERVICE_EXISTS=true
fi

# Build the binary
echo "Building ${SERVICE_NAME}"
cargo build --release

# Install the binary
echo "Installing binary to ${BINARY_PATH}"
sudo cp target/release/gnome_desktop_fixer $BINARY_PATH
sudo chmod +x $BINARY_PATH

# Create systemd user service only if it doesn't exist
if [ "$SERVICE_EXISTS" = false ]; then
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
else
    echo "Service already exists, skipping service setup."
fi

echo "Installation completed!"
echo "Service status: sudo systemctl status gnome-desktop-fixer.service"
echo "Binary available at: $BINARY_PATH"
echo ""
echo "To add the binary to your PATH, add this line to your shell configuration file:"
echo "export PATH=\"/usr/local/bin:\$PATH\""
echo ""
echo "For zsh users: echo 'export PATH=\"/usr/local/bin:\$PATH\"' >> ~/.zshrc"
echo "For bash users: echo 'export PATH=\"/usr/local/bin:\$PATH\"' >> ~/.bashrc"
echo "Then restart your terminal or run 'source ~/.zshrc' (or ~/.bashrc)" 
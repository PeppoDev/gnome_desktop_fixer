#!/bin/bash

SERVICE_NAME="gnome-desktop-fixer"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
BINARY_PATH="/usr/local/bin/${SERVICE_NAME}"

echo "Stopping and disabling systemd service..."
sudo systemctl stop "${SERVICE_NAME}.service"
sudo systemctl disable "${SERVICE_NAME}.service"

echo "Removing systemd service file..."
if [ -f "$SERVICE_FILE" ]; then
    sudo rm "$SERVICE_FILE"
else
    echo "Service file not found: $SERVICE_FILE"
fi

echo "Reloading systemd daemon..."
sudo systemctl daemon-reexec
sudo systemctl daemon-reload

echo "Removing binary..."
if [ -f "$BINARY_PATH" ]; then
    sudo rm "$BINARY_PATH"
else
    echo "Binary not found: $BINARY_PATH"
fi

echo "Uninstallation completed!"
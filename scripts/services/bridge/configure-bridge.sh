#!/usr/bin/env bash

# Define the directory and file paths
MOVEMENT_DIR="./.movement"
CONFIG_FILE="$MOVEMENT_DIR/config.yaml"

NEW_ACCOUNT="0xA550C18"

if [ ! -d "$MOVEMENT_DIR" ]; then
  echo "Error: Directory $MOVEMENT_DIR not found."
  exit 1
fi

if [ ! -f "$CONFIG_FILE" ]; then
  echo "Error: File $CONFIG_FILE not found."
  exit 1
fi

# Use sed to update the account field in the config.yaml file
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS (BSD sed)
    sed -i '' "s/^    account: .*/    account: ${NEW_ACCOUNT}/" "$CONFIG_FILE"
else
    # Linux (GNU sed)
    sed -i "s/^    account: .*/    account: ${NEW_ACCOUNT}/" "$CONFIG_FILE"
fi

echo "Account field updated with value: ${NEW_ACCOUNT}"

# Makefile

.PHONY: all ios-sim

# Variables
APP_NAME := $(shell cat Cargo.toml | dasel -r toml '.package.name' | xargs)
BUNDLE_ID := $(shell cat Cargo.toml | dasel -r toml '.package.metadata.bundle.identifier')

# Default target
all: build-ios-sim build

# Build for iOS Simulator
build-ios-sim:
	cargo bundle --target aarch64-apple-ios-sim


build:
	cargo build

# Install and run on iOS Simulator
ios-sim: build-ios-sim
	xcrun simctl install booted "target/aarch64-apple-ios-sim/debug/bundle/ios/$(APP_NAME).app"
	xcrun simctl launch --console booted $(BUNDLE_ID)


desktop: build
	cargo run
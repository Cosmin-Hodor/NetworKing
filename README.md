# IP Network Scanner

## Overview

A robust, high-performance Rust-based network scanning tool designed to discover and catalog accessible IP addresses across specified network ranges. This application provides comprehensive IP scanning capabilities with geolocation intelligence and MongoDB integration.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![MongoDB](https://img.shields.io/badge/MongoDB-4EA94B?style=for-the-badge&logo=mongodb&logoColor=white)
![GitHub License](https://img.shields.io/github/license/yourusername/ip-network-scanner)

## 🌟 Features

- 🔍 Comprehensive IP range scanning
- 🌐 Geolocation lookup for discovered IPs
- 💾 MongoDB storage for scan results
- 🚀 High-performance concurrent scanning
- 📊 Detailed logging and progress tracking
- 🔒 Configurable scan parameters
- 🔄 Continuous scanning mode

## 🛠 Prerequisites

- Rust (latest stable version)
- MongoDB Atlas account
- Git

## 🚀 Quick Start

### 1. Clone the Repository
```bash
git clone https://github.com/yourusername/ip-network-scanner.git
cd ip-network-scanner
```

### 2. Configure Environment

Create a .env file in the project root:

```
MONGODB_URI=mongodb+srv://<username>:<password>@<cluster-url>/
MONGO_DB_NAME=network_scan_db
MONGO_COLLECTION=accessible_ips
SCAN_PORT=11434
START_IP=106.14.16.0
END_IP=106.14.16.255
SCAN_INTERVAL_MS=200
```

### 3. Install Dependencies
```
cargo build
```

### 4. Run the Scanner
```
cargo run
```


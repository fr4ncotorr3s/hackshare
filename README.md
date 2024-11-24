# Hackshare

`hs` is a command-line tool written in Rust that wraps the `nmap` command, adding caching functionality to speed up repeated scans. If a DNS or IP address has been scanned previously, `hs` retrieves the result from the cache rather than running `nmap` again. This tool is useful for reducing redundant network scans, saving time, and avoiding unnecessary load on the network.

## Features

- **Caching**: Stores `nmap` scan results to avoid re-scanning the same target.
- **Command-Line Interface**: Uses a simple command-line interface for seamless operation.
- **Cache File**: Results are saved in a plain text file (`hs_cache.txt`), with easy access to previously scanned data.

## Requirements

- [Rust](https://www.rust-lang.org/) 1.60+ installed on your machine.
- `nmap` installed on your system.

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/hs.git
   cd hs
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. The compiled binary will be located in the `target/release/` directory.

## Usage

After building the project, you can run the `hs` program with the following syntax:

```bash
hs nmap <nmap-arguments>
```

### Example 1: First Time Scan

Run the following command for a new scan:

```bash
./target/release/hs nmap 192.168.1.1
```

This will run an `nmap` scan on the target `192.168.1.1` and cache the result in `hs_cache.txt`.

### Example 2: Cache Hit

If you run the same command again, the result will be fetched from the cache:

```bash
./target/release/hs nmap 192.168.1.1
```

The cached result will be displayed instead of performing the scan again.

### Example 3: DNS Scan

You can also scan a DNS name:

```bash
./target/release/hs nmap scanme.nmap.org
```

This will run the scan on the `scanme.nmap.org` domain and cache the result.

## Cache Format

The `hs_cache.txt` file stores results in the following format:

```text
192.168.1.1 => Starting Nmap 7.80 ( https://nmap.org ) at Thu Nov 23 15:40:00 2024
Nmap scan report for 192.168.1.1
Host is up (0.00012s latency).
Not shown: 65534 closed ports
PORT     STATE SERVICE
22/tcp   open  ssh
80/tcp   open  http
443/tcp  open  https
12345/tcp open  netbus

Nmap done: 1 IP address (1 host up) scanned in 5.03 seconds

scanme.nmap.org => Starting Nmap 7.80 ( https://nmap.org ) at Thu Nov 23 16:05:12 2024
Nmap scan report for scanme.nmap.org (45.33.32.156)
Host is up (0.0011s latency).
Not shown: 994 filtered ports
PORT     STATE SERVICE
22/tcp   open  ssh
80/tcp   open  http
443/tcp  open  https

Nmap done: 1 IP address (1 host up) scanned in 10.51 seconds
```

Each line consists of the target (IP or DNS) followed by `=>` and the corresponding `nmap` scan result.

## Contributing

Feel free to fork this repository, submit issues, and create pull requests if you'd like to contribute to the project.

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Open a pull request.

### Key Sections

1. **Features**: Highlights the caching and CLI features.
2. **Installation**: Instructions to set up the project.
3. **Usage**: Provides detailed examples of how to use the `hs` tool.
4. **Cache Format**: Describes the format of `hs_cache.txt` to clarify how results are stored.
5. **Contributing**: Explains how others can contribute to the project.

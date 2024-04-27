# Resolve

Resolve is a simple command-line tool written in Rust for resolving DNS records.

## Usage
1. **Clone the Repository:**
   ```bash
   git clone https://github.com/yourusername/dns_resolver.git
   ```

2. **Build the Project:**
   ```bash
   cargo build --release
   ```

3. **Run the Resolver:**
   ```bash
   ./target/release/resolve <domain_to_resolve>
   ```

4. **Example:**
    ```bash
   ./target/release/resolve google.com
   ```

5. **Custom DNS Server:**
    ```bash
     ./target/release/resolve <domain_to_resolve> --dns-server <custom-dns-server-ip>
    ```

6. **Example:**
    ```bash
     ./target/release/resolve google.com --dns-server 8.8.8.8
    ```
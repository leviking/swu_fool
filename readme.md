# SWU Log Fetcher in Rust
This is a Rust CLI tool for fetching logs from SendWithUs and outputting them into a CSV file. The program reads email addresses from an input CSV file and queries the SendWithUs API to get log data associated with those email addresses.

## Features
Fetches logs from the SendWithUs API for a list of email addresses.
Writes the logs to a CSV file for easy analysis and sharing.
## Requirements
Rust and Cargo installed
API key for SendWithUs
## Installation
### From Source
Clone this repository and then navigate to the repository folder:

```
git clone https://github.com/leviking/swu_fool.git
cd swu_fool
```

### Build the project:
```
cargo build --release
```
The executable will be in the ./target/release/ directory.

### Usage
Setting up environment variables
First, set the SWU_KEY environment variable with your SendWithUs API key:

`export SWU_KEY='your-api-key-here'`
### Running the tool
Run the tool by passing the input CSV file containing email addresses as a command-line argument:

```
./swu_fool input.csv
```
This will generate an output CSV file named output.csv containing the logs.

## Contributing
If you have suggestions for how this tool could be improved, please open an issue or a pull request.

## License
This project is open-source, under the MIT License.
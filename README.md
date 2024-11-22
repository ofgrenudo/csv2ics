# CSV to ICS Converter

This Rust program converts a CSV file into ICS files, which can be imported into calendar applications such as Google Calendar, Outlook, or Apple Calendar. It is designed to simplify the process of scheduling events programmatically.

## Features
- Converts a CSV file into ICS (iCalendar) files.
- Automatically generates unique event identifiers (GUIDs).
- Allows you to specify an output directory for the ICS files.
- Includes options to assign a custom company name in the ICS metadata.
- Generates a CSV template for users to modify with their own event data.

## Requirements
- Rust (latest stable version)
- [clap](https://crates.io/crates/clap) for command-line argument parsing.
- [ics](https://crates.io/crates/ics) for iCalendar file generation.
- [serde](https://crates.io/crates/serde) and [csv](https://crates.io/crates/csv) for handling CSV input and data deserialization.
- [uuid](https://crates.io/crates/uuid) for generating unique event IDs.

## Installation
1. Clone this repository:
   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the binary:
   ```bash
   ./target/release/csv_to_ics
   ```

## Usage
This program is controlled via command-line arguments. Below are the available options:

### Command-Line Arguments
- `-i, --input <FILE>`: Path to the input CSV file to convert into ICS files.
- `-o, --output-dir <DIR>`: Output directory where the generated ICS files will be saved. Default: `./events/`.
- `-c, --company <NAME>`: Name of the company to include in ICS metadata. Default: `CSV2ICS`.
- `-t, --template`: Generates a CSV template in the working directory.

### Examples
1. Generate a CSV template:
   ```bash
   csv_to_ics --template
   ```

   This creates a `template.csv` file:
   ```csv
   organizer,start_time,end_time,summary,description
   mailto:events@university.edu,2024-11-25T10:00:00,2024-11-25T12:00:00,Department Meeting,Room 101 - Admin Building
   ```

2. Convert a CSV file to ICS files:
   ```bash
   csv_to_ics --input my_events.csv
   ```

3. Specify an output directory:
   ```bash
   csv_to_ics --input my_events.csv --output-dir ./my_calendar/
   ```

4. Assign a company name in the ICS files:
   ```bash
   csv_to_ics --input my_events.csv --company "My Company"
   ```

## CSV File Format
The CSV file should have the following headers:
- `organizer`: Email or contact info of the organizer (e.g., `mailto:events@company.com`).
- `start_time`: Event start time in ISO 8601 format (`YYYY-MM-DDTHH:MM:SS`).
- `end_time`: Event end time in ISO 8601 format (`YYYY-MM-DDTHH:MM:SS`).
- `summary`: A short description of the event (e.g., "Team Meeting").
- `description`: Detailed information about the event.

### Example CSV
```csv
organizer,start_time,end_time,summary,description
mailto:events@university.edu,2024-11-25T10:00:00,2024-11-25T12:00:00,Department Meeting,Room 101 - Admin Building
mailto:john.doe@company.com,2024-12-01T14:00:00,2024-12-01T15:00:00,Project Kickoff,Zoom Meeting Link
```

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributions
Contributions are welcome! Feel free to fork the repository and submit a pull request.

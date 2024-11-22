use ics::properties::{Categories, Description, DtEnd, DtStart, Organizer, Status, Summary};
use ics::{escape_text, Event, ICalendar};
use serde::{Deserialize, Serialize};
use std::{error::Error, io, fs, path::Path};
use csv::ReaderBuilder;
use uuid::Uuid;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;


/// Convert a CSV file to ICS files to import into your calendar!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Target CSV file to consume and convert into
    /// ICS files.
    #[arg(short, long)]
    input: Option<String>,

    /// Output DIR on where to place CSV Files.
    /// Default value is a events folder in your
    /// working DIR.
    #[arg(short, long, default_value = "./events/")]
    output_dir: Option<String>,

    /// Assign a Company Name in the ICS Files
    /// Default value is CSV2ICS
    #[arg(short, long, default_value = "CSV2ICS")]
    company: Option<String>,

    /// Produces a template for you in the working
    /// directory to modify with your own data!
    #[arg(short, long)]
    template: bool,

}

#[derive(Serialize, Deserialize, Debug)]
struct CsvEventData {
    organizer: String,
    start_time: String,
    end_time: String,
    summary: String,
    description: String,
}

#[derive(Debug)]
struct EventData {
    guid: String,
    organizer: String,
    start_time: String,
    end_time: String,
    summary: String,
    description: String,
}

fn ensure_directory_exists(dir_path: &str) -> std::io::Result<()> {
    let path = Path::new(dir_path);

    if !path.exists() {
        // println!("Directory does not exist. Creating: {}", dir_path);
        fs::create_dir_all(path)?; // Create the directory and any necessary parent directories
    } else {
        // println!("Directory already exists: {}", dir_path);
        ;
    }

    Ok(())
}

fn process_csv<R: io::Read>(reader: R) -> Result<Vec<EventData>, Box<dyn Error>> {
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);

    let mut events = Vec::new();
    for result in csv_reader.deserialize() {
        let csv_event: CsvEventData = result?; // Deserialize into CsvEventData
        let mut event = EventData {
            guid: Uuid::new_v4().to_string(), // Generate GUID dynamically
            organizer: csv_event.organizer,
            start_time: csv_event.start_time.replace(&['-', ':'], ""),
            end_time: csv_event.end_time.replace(&['-', ':'], ""),
            summary: csv_event.summary,
            description: csv_event.description,
        };

        // event.start_time = event.start_time.replace(&['-', ':'], "");
        events.push(event);
    }
    Ok(events)
}

fn create_calendar_event(event_data: EventData, company_name: &String, output_dir: &String) -> std::io::Result<()> {
    let mut calendar = ICalendar::new("2.0", format!("-//{}//NONSGML PDA Calendar Version 1.0//EN", company_name));
    let mut event = Event::new(event_data.guid, &event_data.start_time);

    event.push(Organizer::new(event_data.organizer));
    event.push(DtStart::new(&event_data.start_time));
    event.push(DtEnd::new(event_data.end_time));
    event.push(Status::confirmed());
    event.push(Categories::new("Ticket"));
    event.push(Summary::new(&event_data.summary));
    event.push(Description::new(escape_text(event_data.description)));

    calendar.add_event(event);

    ensure_directory_exists(output_dir)?;

    let save_path = format!("{}{}.ics", &output_dir, &event_data.summary.replace(&['/', '\\'], ""));
    
    calendar.save_file(save_path)?;
    
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Match on the arguments provided
    if args.template {
        // Handle the `get_template` flag
        let template_content = "organizer,start_time,end_time,summary,description\nmailto:events@university.edu,2024-11-25T10:00:00,2024-11-25T12:00:00,Department Meeting,Room 101 - Admin Building";
        fs::write("template.csv", template_content)?;
        println!("Template CSV file created: template.csv");
    } else if let Some(input_csv) = args.input {
        // Handle the `input_csv` argument
        println!("Converting CSV file: {}\n", input_csv);

        let file = File::open(&input_csv)?;
        let reader = BufReader::new(file);

        // Process the CSV file
        let events = process_csv(reader)?;

        for event in events {
            println!("Creating Event {}",  &event.summary);
            create_calendar_event(event, &args.company.as_ref().unwrap(), &args.output_dir.as_ref().unwrap())?;
        }

    } else {
        // If no valid arguments were provided, show a help message
        eprintln!("No valid arguments provided. Use --help to see usage.");
    }

    Ok(())
}


pub mod operations {
    use slug::slugify;
    use std::error::Error;

    use std::str::FromStr;

    pub const AVAILABLE_OPERATIONS: [&str; 7] = [
        "lowercase",
        "uppercase",
        "no-spaces",
        "slugify",
        "reverse",
        "title-case",
        "csv",
    ];

    // Defined the enum with allowed values from string modifications
    pub enum StringOperation {
        Lowercase,
        Uppercase,
        NoSpaces,
        Slugify,
        Reverse,
        TitleCase,
        Csv,
    }

    impl FromStr for StringOperation {
        // Read the string from arg and convert it to the enum
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "uppercase" => Ok(StringOperation::Uppercase),
                "lowercase" => Ok(StringOperation::Lowercase),
                "reverse" => Ok(StringOperation::Reverse),
                "no-spaces" => Ok(StringOperation::NoSpaces),
                "slugify" => Ok(StringOperation::Slugify),
                "title-case" => Ok(StringOperation::TitleCase),
                "csv" => Ok(StringOperation::Csv),
                _ => Err(format!(
                    "Unknown modification: {s}\nAvailable modifications: {mods}",
                    s = s,
                    mods = AVAILABLE_OPERATIONS.join(", ")
                )),
            }
        }
    }

    fn is_valid_string(input: &str) -> (bool, &str) {
        return (!input.trim().is_empty(), "string_validation");
    }

    fn get_input(input_prompt: String) -> Result<String, Box<dyn Error>> {
        println!("{}\n", input_prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn get_valid_input(
        validators: Vec<fn(&str) -> (bool, &str)>,
        input_prompt: String,
    ) -> Result<String, Box<dyn Error>> {
        let input = get_input(input_prompt)?;

        for validator in validators {
            let (valid, validation_type) = validator(&input);

            if !valid {
                return Err(format!("Invalid input for validator: {}", validation_type).into());
            }
        }
        Ok(input)
    }

    // Define the functions for each modification
    fn lowercase(input: Option<&str>) -> Result<String, Box<dyn Error>> {
        match input {
            Some(input) => Ok(input.to_lowercase()),
            None => {
                let input: String = get_valid_input(
                    vec![is_valid_string],
                    String::from("Provide the string to modify: "),
                )?;
                Ok(input.to_lowercase())
            }
        }
    }

    fn uppercase(input: Option<&str>) -> Result<String, Box<dyn Error>> {
        match input {
            Some(input) => Ok(input.to_uppercase()),
            None => {
                let input: String = get_valid_input(
                    vec![is_valid_string],
                    String::from("Provide the string to modify: "),
                )?;
                Ok(input.to_uppercase())
            }
        }
    }

    fn no_spaces(input: Option<&str>) -> Result<String, Box<dyn Error>> {
        match input {
            Some(input) => Ok(input.replace(" ", "")),
            None => {
                let input: String = get_valid_input(
                    vec![is_valid_string],
                    String::from("Provide the string to modify: "),
                )?;
                Ok(input.replace(" ", ""))
            }
        }
    }

    fn slugify_input(input: Option<&str>) -> Result<String, Box<dyn Error>> {
        match input {
            Some(input) => Ok(slugify(input)),
            None => {
                let input: String = get_valid_input(
                    vec![is_valid_string],
                    String::from("Provide the string to modify: "),
                )?;
                Ok(slugify(input))
            }
        }
    }

    fn reverse(input: Option<&str>) -> Result<String, Box<dyn Error>> {
        match input {
            Some(input) => Ok(input.chars().rev().collect()),
            None => {
                let input: String = get_valid_input(
                    vec![is_valid_string],
                    String::from("Provide the string to modify: "),
                )?;
                Ok(input.chars().rev().collect())
            }
        }
    }

    fn get_title_case(input: &str) -> Result<String, Box<dyn Error>> {
        Ok(input
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first_char) => first_char
                        .to_uppercase()
                        .chain(chars.flat_map(|c| c.to_lowercase()))
                        .collect(),
                }
            })
            .collect::<Vec<String>>()
            .join(" "))
    }

    fn title_case(input: Option<&str>) -> Result<String, Box<dyn Error>> {
        match input {
            Some(input) => Ok(get_title_case(input)?),
            None => {
                // If no input is provided, ask for it
                let input: String = get_valid_input(
                    vec![is_valid_string],
                    String::from("Provide the string to modify: "),
                )?;
                Ok(get_title_case(&input)?)
            }
        }
    }

    pub fn get_modified_input(
        input: Option<&str>,
        operation: StringOperation,
    ) -> Result<String, Box<dyn Error>> {
        // Match the enum variant and call the corresponding function
        let modified_input = match operation {
            StringOperation::Lowercase => lowercase(input),
            StringOperation::Uppercase => uppercase(input),
            StringOperation::NoSpaces => no_spaces(input),
            StringOperation::Slugify => slugify_input(input),
            StringOperation::Reverse => reverse(input),
            StringOperation::TitleCase => title_case(input),
            StringOperation::Csv => csv_operations::parse_as_csv(input),
        };
        Ok(modified_input?)
    }

    pub mod csv_operations {
        use super::{get_valid_input, is_valid_string};
        use csv::StringRecord;
        use std::error::Error;
        use std::fmt;

        pub struct CsvRecords {
            pub headers: StringRecord,
            pub records: Vec<csv::StringRecord>,
        }

        impl fmt::Display for CsvRecords {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut column_widths = vec![0; self.headers.len()];

                for (i, header) in self.headers.iter().enumerate() {
                    column_widths[i] = header.len();
                }

                for record in &self.records {
                    for (i, field) in record.iter().enumerate() {
                        if field.len() > column_widths[i] {
                            column_widths[i] = field.len();
                        }
                    }
                }

                let format_row = |row: &StringRecord| {
                    row.iter()
                        .enumerate()
                        .map(|(i, field)| format!("{:width$}", field, width = column_widths[i]))
                        .collect::<Vec<String>>()
                        .join(" | ")
                };

                writeln!(f, "{}", format_row(&self.headers))?;
                writeln!(
                    f,
                    "{}",
                    column_widths
                        .iter()
                        .map(|&w| "-".repeat(w))
                        .collect::<Vec<String>>()
                        .join("-+-")
                )?;

                // Print records
                for record in &self.records {
                    writeln!(f, "{}", format_row(record))?;
                }

                Ok(())
            }
        }

        fn process_csv_from_reader(
            reader: &mut csv::Reader<std::fs::File>,
        ) -> Result<String, Box<dyn Error>> {
            let mut records: Vec<csv::StringRecord> = Vec::new();

            for result in reader.records() {
                let record = result?;
                records.push(record);
            }
            let headers = reader.headers()?.clone();
            let csv_records = CsvRecords { headers, records };

            Ok(csv_records.to_string())
        }

        pub fn parse_as_csv(file_path: Option<&str>) -> Result<String, Box<dyn Error>> {
            match file_path {
                Some(file_path) => {
                    let mut rdr = csv::Reader::from_path(file_path)?;
                    Ok(process_csv_from_reader(&mut rdr)?)
                }
                None => {
                    match get_valid_input(
                        vec![is_valid_string],
                        "Please insert the path to the CSV file:".to_string(),
                    ) {
                        Ok(input) => {
                            print!("Reading CSV file... {}\n", &input);
                            let mut rdr = csv::Reader::from_path(&input)?;
                            Ok(process_csv_from_reader(&mut rdr)?)
                        }
                        Err(e) => return Err(e.into()),
                    }
                }
            }
        }
    }
}

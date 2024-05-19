pub mod modifications {
    use slug::slugify;
    use std::error::Error;
    use std::str::FromStr;

    pub const AVAILABLE_MODIFICATIONS: [&str; 6] = [
        "lowercase",
        "uppercase",
        "no-spaces",
        "slugify",
        "reverse",
        "title-case",
    ];

    // Defined the enum with allowed values from string modifications
    enum StringModification {
        Lowercase,
        Uppercase,
        NoSpaces,
        Slugify,
        Reverse,
        TitleCase,
    }

    impl FromStr for StringModification {
        // Read the string from arg and convert it to the enum
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "uppercase" => Ok(StringModification::Uppercase),
                "lowercase" => Ok(StringModification::Lowercase),
                "reverse" => Ok(StringModification::Reverse),
                "no-spaces" => Ok(StringModification::NoSpaces),
                "slugify" => Ok(StringModification::Slugify),
                "title-case" => Ok(StringModification::TitleCase),
                _ => Err(format!(
                    "Unknown modification: {s}\nAvailable modifications: {mods}",
                    s = s,
                    mods = AVAILABLE_MODIFICATIONS.join(", ")
                )),
            }
        }
    }

    fn is_valid_string(input: &str) -> (bool, &str) {
        return (!input.trim().is_empty(), "string_validation");
    }

    fn get_input(input_prompt: String) -> Result<String, Box<dyn Error>> {
        println!("{}", input_prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input)
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
    fn lowercase() -> Result<String, Box<dyn Error>> {
        let input: String = get_valid_input(
            vec![is_valid_string],
            String::from("Provide the string to modify: "),
        )?;
        Ok(input.to_lowercase())
    }

    fn uppercase() -> Result<String, Box<dyn Error>> {
        let input: String = get_valid_input(
            vec![is_valid_string],
            String::from("Provide the string to modify: "),
        )?;
        Ok(input.to_uppercase())
    }

    fn no_spaces() -> Result<String, Box<dyn Error>> {
        let input: String = get_valid_input(
            vec![is_valid_string],
            String::from("Provide the string to modify: "),
        )?;
        Ok(input.replace(" ", ""))
    }

    fn slugify_input() -> Result<String, Box<dyn Error>> {
        let input: String = get_valid_input(
            vec![is_valid_string],
            String::from("Provide the string to modify: "),
        )?;
        Ok(slugify(input))
    }

    fn reverse() -> Result<String, Box<dyn Error>> {
        let input: String = get_valid_input(
            vec![is_valid_string],
            String::from("Provide the string to modify: "),
        )?;
        Ok(input.chars().rev().collect())
    }

    fn title_case() -> Result<String, Box<dyn Error>> {
        let input: String = get_valid_input(
            vec![is_valid_string],
            String::from("Provide the string to modify: "),
        )?;
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

    pub fn get_modified_input(raw_modification: &str) -> Result<String, Box<dyn Error>> {
        let modification = StringModification::from_str(raw_modification)?;
        // Match the enum variant and call the corresponding function
        let modified_input = match modification {
            StringModification::Lowercase => lowercase(),
            StringModification::Uppercase => uppercase(),
            StringModification::NoSpaces => no_spaces(),
            StringModification::Slugify => slugify_input(),
            StringModification::Reverse => reverse(),
            StringModification::TitleCase => title_case(),
        };
        Ok(modified_input?)
    }
}

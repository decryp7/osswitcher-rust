use std::collections::HashMap;
use std::error::Error;
use std::process::{Command, Stdio};

use regex::Regex;

#[derive(Debug)]
pub struct OS {
    pub identifier: String,
    pub device: String,
    pub path: String,
    pub description: String,
    pub locale: String,
}

impl OS {
    // fn new() -> OS {
    //     OS {
    //         identifier: "".to_string(),
    //         device: "".to_string(),
    //         path: "".to_string(),
    //         description: "".to_string(),
    //         locale: "".to_string(),
    //     }
    // }
}

pub trait OSSwitcher {
    fn get_os_options(&mut self) -> Result<&HashMap<u32, OS>, Box<dyn Error>>;
    fn switch_os(&self, option: u32) -> Result<(), Box<dyn Error>>;
}

pub struct BCDOSSwitcher {
    os_options: HashMap<u32, OS>,
}

impl BCDOSSwitcher {
    pub fn new() -> BCDOSSwitcher {
        BCDOSSwitcher {
            os_options: HashMap::new(),
        }
    }

    fn parse_os(os_string: &str) -> OS {
        let properties = os_string.split("\r\n");
        let mut property_values: HashMap<&str, &str> = HashMap::new();
        for property in properties {
            //println!("{}", property);
            let mut property_value = property.split_whitespace();
            let key = property_value.next().unwrap_or("");
            let value = property_value.next().unwrap_or("");
            property_values.insert(key, value);
        }

        OS {
            identifier: property_values.get("identifier").unwrap().to_string(),
            device: property_values.get("device").unwrap().to_string(),
            path: property_values.get("path").unwrap().to_string(),
            description: property_values.get("description").unwrap().to_string(),
            locale: property_values.get("locale").unwrap().to_string(),
        }
    }
}

impl OSSwitcher for BCDOSSwitcher {
    fn get_os_options(&mut self) -> Result<&HashMap<u32, OS>, Box<dyn Error>> {
        let cmd_output = Command::new("bcdedit")
            .output()
            .expect("Failed to run bcdedit");

        let result = String::from_utf8_lossy(&cmd_output.stdout);
        //println!("{:?}", result);
        let os_strings = result.split("\r\n\r\n");
        //println!("{:?}",test.next().unwrap());

        let regex = Regex::new(r"(?mi)^\{[0-9a-f]{8}-([0-9a-f]{4}\-){3}[0-9a-f]{12}\}$").unwrap();
        let mut count = 0;
        for os_string in os_strings {
            let os = BCDOSSwitcher::parse_os(os_string);
            if regex.is_match(&os.identifier) {
                //println!("{:?}", os);
                count = count + 1;
                self.os_options.insert(count, os);
            }
        }

        Ok(&self.os_options)
    }

    fn switch_os(&self, option: u32) -> Result<(), Box<dyn Error>> {
        if !self.os_options.contains_key(&option) {
            Err("The choice is invalid")?
        }

        //set select os timeout to 2 seconds
        Command::new("bcdedit")
            .arg("/timeout")
            .arg("2")
            .stdout(Stdio::inherit())
            .output()
            .expect("Failed to run bcdedit /timeout 2");

        Command::new("bcdedit")
            .arg("/default")
            .arg(&self.os_options[&option].identifier)
            .stdout(Stdio::inherit())
            .output()
            .expect(
                format!(
                    "Failed to run bcdedit /default {}",
                    self.os_options[&option].identifier
                )
                .as_str(),
            );

        Command::new("shutdown")
            .arg("-r")
            .arg("-t")
            .arg("0")
            .stdout(Stdio::inherit())
            .output()
            .expect("Failed to reboot!");

        Ok(())
    }
}

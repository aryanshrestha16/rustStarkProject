#[derive(Debug)]
#[derive(PartialEq)]
pub enum Command
{
    Power(bool,i32),    // [Increase/Decrease] power by [number].
    Missiles(bool,i32), // [Increase/Decrease] missiles by [number].
    Shield(bool),       // Turn [On/Off] the shield.
    Try,                // Try calling pepper.
    Invalid             // [anything else]
}


/**
    Adds functionality to Command enums
    Commands can be converted to strings with the as_str method
    
    Command     |     String format
    ---------------------------------------------------------
    Power       |  /Power (increased|decreased) by [0-9]+%/
    Missiles    |  /Missiles (increased|decreased) by [0-9]+/
    Shield      |  /Shield turned (on|off)/
    Try         |  /Call attempt failed/
    Invalid     |  /Not a command/
**/
impl Command {
    pub fn as_str (&self) -> String {
        match self {
            Command::Power(true,x) => format!("{}{}{}",String::from("Power increased by "),*x,String::from("%")),
            Command::Power(false,x) => format!("{}{}{}",String::from("Power decreased by "),*x,String::from("%")),
            Command::Missiles(true,x) => format!("{}{}",String::from("Missiles increased by "),*x),
            Command::Missiles(false,x) => format!("{}{}",String::from("Missiles decreased by "),*x),
            Command::Shield(true) => String::from("Shield turned on"),
            Command::Shield(false) => String::from("Shield turned off"),
            Command::Try => String::from("Call attempt failed"),
            Command::Invalid => String::from("Not a command"),
        }
    }
}

/**
    Complete this method that converts a string to a command 
    We list the format of the input strings below

    Command     |     String format
    ---------------------------------------------
    Power       |  /power (inc|dec) [0-9]+/
    Missiles    |  /(fire|add) [0-9]+ missiles/
    Shield      |  /shield (on|off)/
    Try         |  /try calling Miss Potts/
    Invalid     |  Anything else
**/
pub fn to_command(s: &str) -> Command {
    let mut a = s.strip_prefix("power inc ");
    match a {
        Some (x) =>  match x.parse::<i32>() {
            Ok(n) => return Command::Power(true,n),
            _ => return Command::Invalid,
        }
        None => {
            a = s.strip_prefix("power dec ");
            match a {
                Some (x) => match x.parse::<i32>() {
                    Ok(n) => return Command::Power(false,n),
                    _ => return Command::Invalid,
                },
                None => {
                    a = s.strip_prefix("fire ");
                    match a {
                        Some (x) => 
                            match x.strip_suffix(" missiles") {
                                Some(x) => match x.parse::<i32>() {
                                    Ok(n) => return Command::Missiles(false,n),
                                    _ => return Command::Invalid,
                                }
                                None => return Command::Invalid,
                            },
                        None => {
                            a = s.strip_prefix("add ");
                            match a {
                                Some (x) => match x.strip_suffix(" missiles") {
                                    Some(x) => match x.parse::<i32>() {
                                        Ok(n) => return Command::Missiles(true,n),
                                        _ => return Command::Invalid,
                                    }
                                    None => return Command::Invalid,
                                },
                                None =>{
                                    a = s.strip_prefix("shield on");
                                    match a {
                                        Some (x) => if x == "" {
                                            return Command::Shield(true);
                                        }
                                        else {
                                            return Command::Invalid;
                                        },
                                        None => {
                                            a = s.strip_prefix("shield off");
                                            match a {
                                                Some (x) => if x == "" {
                                                    return Command::Shield(false);
                                                }
                                                else {
                                                    return Command::Invalid;
                                                },
                                                None => {
                                                    a = s.strip_prefix("try calling Miss Potts");
                                                    match a {
                                                        Some (x) => if x == "" {
                                                            return Command::Try;
                                                        }
                                                        else {
                                                            return Command::Invalid;
                                                        },
                                                        None => return Command::Invalid,
                                                    }
                                                },
                                            }
                                        },
                                    }
                                },
                            }

                        },
                    }
                },
            }
        },
    }
}

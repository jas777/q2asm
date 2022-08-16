use std::collections::HashMap;

pub fn find_labels(lines: &Vec<&str>) -> Result<HashMap<String, usize>, String> {
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut line_num: usize = 0;
    let mut actual_line_num: usize = 0;
    for line in lines {
        line_num += 1;
        actual_line_num += 1;

        let line = line.trim();

        if line.is_empty() || line.starts_with("#") || line.starts_with("--") {
            line_num -= 1;
            continue;
        }

        if line.starts_with(".") {
            let label_name: Vec<&str> = line.split("--").collect();
            let label_name = label_name[0];
            let stripped_name = label_name.strip_prefix(".");

            // println!("{}", &label_name.unwrap());
            match stripped_name {
                Some(name) => {
                    if name.contains(" ") {
                        return Err(format!(
                            "Syntax error on line {}! A label can't contain any spaces!",
                            actual_line_num
                        ));
                    }

                    if labels.contains_key(name.trim()) {
                        return Err(
                            format!(
                                "Error on line {}! Label with the name \"{}\" has already been defined!",
                                actual_line_num, name.trim()
                            )
                        );
                    }

                    line_num -= 1;
                    println!("Mapping label \"{}\" to {:0b}", name.trim(), line_num);
                    labels.insert(name.to_string(), line_num);
                    continue;
                }
                None => {
                    return Err(format!(
                        "Syntax error on line {}! A label must have a name!",
                        actual_line_num
                    ));
                }
            }
        }
    };
    Ok(labels)
}

pub fn find_constants(lines: &Vec<&str>) -> Result<HashMap<String, usize>, String> {
    let mut constants: HashMap<String, usize> = HashMap::new();
    let mut actual_line_num: usize = 0;
    for line in lines {
        actual_line_num += 1;

        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("#DEFINE") {
            let const_def: Vec<&str> = line.split("--").collect();
            let const_def = const_def[0].trim();

            let const_split: Vec<&str> = const_def.split("=").collect();
            let const_name = const_split[0].strip_prefix("#DEFINE");

            // println!("{}", &label_name.unwrap());
            match const_name {
                Some(name) => {
                    if name.trim().contains(" ") {
                        return Err(format!(
                            "Syntax error on line {}! A const name can't contain any spaces!",
                            actual_line_num
                        ));
                    }

                    if constants.contains_key(name.trim()) {
                        return Err(
                            format!(
                                "Error on line {}! Constant with the name \"{}\" has already been defined!",
                                actual_line_num, name.trim()
                            )
                        );
                    }

                    let mut val_trimmed = const_split[1].trim().to_string();
                    if val_trimmed.starts_with("-") {
                        val_trimmed = match handle_negative_val(val_trimmed) {
                            Ok(val) => val,
                            Err(e) => return Err(e)
                        }
                    }

                    let const_val: usize = match parse_int::parse(&val_trimmed) {
                        Ok(val) => val,
                        Err(_) => return Err(format!("Invalid const value! Const: \"{}\"", name.trim()))
                    };

                    println!("Mapping const \"{}\" to {:0b}", name.trim(), const_val);
                    constants.insert(name.trim().to_string(), const_val);
                    continue;
                }
                None => {
                    return Err(format!(
                        "Syntax error on line {}! A const must have a name!",
                        actual_line_num
                    ));
                }
            }
        }
    };
    Ok(constants)
}

pub fn handle_negative_val(value: String) -> Result<String, String> {
    let val_trimmed_stripped = value.strip_prefix("-").unwrap();
    let val: usize = match parse_int::parse(val_trimmed_stripped) {
        Ok(val) => val,
        Err(_) => return Err(format!("Invalid const value! Const: \"{}\"", ""))
    };

    let target_len = format!("{:0b}", val).len();
    let negated = format!("{:0target_len$b}", val - 1)
        .replace("1", "2")
        .replace("0", "1")
        .replace("2", "0");

    let u2_rep = format!("0b{}", negated);
    Ok(u2_rep)
}

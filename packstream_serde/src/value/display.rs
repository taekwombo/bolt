use std::collections::HashMap;
use std::fmt;

pub fn display_value_list<V: fmt::Display>(value: &Vec<V>, formatter: &mut fmt::Formatter) -> fmt::Result {
    let len = value.len();

    if len == 0 {
        return formatter.write_str("[]");
    }

    let mut output = String::with_capacity(len * 4);

    output.push('[');

    value.iter().enumerate().for_each(|(index, value)| {
        if index + 1 == len {
            output.push_str(&format!("{}", value));
        } else {
            output.push_str(&format!("{}, ", value));
        }
    });

    output.push(']');

    formatter.write_str(output.as_ref())
}

pub fn display_value_hash_map<V: fmt::Display>(value: &HashMap<String, V>, formatter: &mut fmt::Formatter) -> fmt::Result {
    let len = value.len();

    if len == 0 {
        return formatter.write_str("{}");
    }

    let mut output = String::with_capacity(len * 8);

    output.push('{');

    value.iter().enumerate().for_each(|(index, (key, value))| {
        if index + 1 == len {
            output.push_str(&format!(" {}: {}", key, value));
        } else {
            output.push_str(&format!(" {}: {},", key, value));
        }
    });

    output.push_str(" }");

    formatter.write_str(output.as_ref())
}

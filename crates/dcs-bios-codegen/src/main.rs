use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ControlDefinition {
    identifier: String,
    control_type: String,
    description: Option<String>,
    #[serde(default)]
    inputs: Vec<InputDefinition>,
    #[serde(default)]
    outputs: Vec<OutputDefinition>,
}

#[derive(Debug, Deserialize)]
struct InputDefinition {
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OutputDefinition {
    address: u16,
    description: Option<String>,
    #[serde(rename = "type")]
    output_type: String,
    max_value: Option<u16>,
    mask: Option<u16>,
    shift_by: Option<u8>,
    max_length: Option<usize>,
    suffix: Option<String>,
}

type Model = BTreeMap<String, BTreeMap<String, ControlDefinition>>;

struct GeneratedControl {
    name: String,
    group_module: String,
    description: String,
    base: String,
    callback: String,
    callback_argument: String,
    commit_callback: String,
    constructor: String,
    accessor: &'static str,
    enum_definition: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .ok_or("could not determine workspace directory")?;

    let mut input_dir = workspace_dir.join("model-definitions");
    let mut output_dir = workspace_dir.join("crates/generated/src");
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input-dir" => {
                input_dir = PathBuf::from(args.next().ok_or("missing --input-dir value")?)
            }
            "--output-dir" => {
                output_dir = PathBuf::from(args.next().ok_or("missing --output-dir value")?)
            }
            "-h" | "--help" => {
                println!("Usage: dcs-bios-codegen [--input-dir DIR] [--output-dir DIR]");
                return Ok(());
            }
            other => return Err(format!("unknown argument: {other}").into()),
        }
    }

    let mut generated_files = Vec::new();
    for entry in fs::read_dir(&input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
            continue;
        }
        let model_name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or("model definition has no valid file name")?;
        let module_name = module_case(model_name);
        let source = generate_model(&fs::read_to_string(&path)?, model_name)?;
        generated_files.push((module_name, source));
    }

    fs::create_dir_all(&output_dir)?;
    for entry in fs::read_dir(&output_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            fs::remove_file(path)?;
        }
    }

    let mut modules = generated_files
        .iter()
        .map(|(module_name, _)| module_name.clone())
        .collect::<Vec<_>>();
    for (module_name, source) in generated_files {
        fs::write(output_dir.join(format!("{module_name}.rs")), source)?;
    }
    modules.sort();

    let mut lib = String::from("#![no_std]\n\n//! Generated DCS-BIOS aircraft bindings.\n\n");
    for module in modules {
        writeln!(lib, "pub mod {module};")?;
    }
    fs::write(output_dir.join("lib.rs"), lib)?;
    Ok(())
}

fn generate_model(json: &str, model_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let model: Model = serde_json::from_str(json)?;
    let mut controls = Vec::new();
    let mut used_names = BTreeSet::new();
    for (group_name, group) in &model {
        let group_module = module_case(group_name);
        for definition in group.values() {
            for (index, output) in definition.outputs.iter().enumerate() {
                if let Some(mut control) = generated_control(definition, output, index)? {
                    control.group_module = group_module.clone();
                    if used_names.contains(&control.name) {
                        let base_name = control.name.clone();
                        let group_suffix = pascal_case(group_name);
                        control.name = format!("{base_name}{group_suffix}");
                        let mut suffix_number = 2;
                        while used_names.contains(&control.name) {
                            control.name = format!("{base_name}{group_suffix}{suffix_number}");
                            suffix_number += 1;
                        }
                        let old_value_name = format!("{base_name}Value");
                        let new_value_name = format!("{}Value", control.name);
                        control.callback =
                            control.callback.replace(&old_value_name, &new_value_name);
                        control.callback_argument = control
                            .callback_argument
                            .replace(&old_value_name, &new_value_name);
                        control.commit_callback = control
                            .commit_callback
                            .replace(&old_value_name, &new_value_name);
                        if let Some(enum_definition) = &mut control.enum_definition {
                            *enum_definition =
                                enum_definition.replace(&old_value_name, &new_value_name);
                        }
                    }
                    used_names.insert(control.name.clone());
                    controls.push(control);
                }
            }
        }
    }
    controls.sort_by(|left, right| left.name.cmp(&right.name));

    let aircraft_name = pascal_case(model_name);
    let mut source = String::new();
    writeln!(
        source,
        "//! Generated from model-definitions/{model_name}.json."
    )?;
    writeln!(
        source,
        "//! Do not edit manually; rerun dcs-bios-codegen instead.\n"
    )?;
    source.push_str("#![allow(unused_imports)]\n\n");
    source.push_str(
        "use core::marker::PhantomData;\n\
         use heapless::Vec;\n\
         use dcs_bios_rs_lib::{AnalogDial, AnalogGauge, Control, Display, FeedError, FixedStepDial,\n\
         Led, LimitedDial, Metadata, Packet, ProtocolParser, Selector, Sink};\n\
         \n",
    );
    source.push_str("pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);\n");
    source.push_str("pub type LedCallback<'a> = &'a mut dyn FnMut(bool);\n");
    source.push_str("pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);\n\n");

    let mut groups = BTreeMap::<String, Vec<&GeneratedControl>>::new();
    for control in &controls {
        groups
            .entry(control.group_module.clone())
            .or_default()
            .push(control);
    }
    for (group_module, group_controls) in groups {
        writeln!(source, "pub mod {group_module} {{")?;
        source.push_str("    use super::*;\n\n");
        source.push_str("    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);\n");
        source.push_str("    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);\n");
        source.push_str("    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);\n\n");
        for control in group_controls {
            if let Some(enum_definition) = &control.enum_definition {
                for line in enum_definition.lines() {
                    writeln!(source, "    {line}")?;
                }
                source.push('\n');
            }
            for line in control.description.lines() {
                writeln!(source, "    /// {line}")?;
            }
            writeln!(source, "    pub struct {}({});", control.name, control.base)?;
            writeln!(source, "    impl {} {{", control.name)?;
            writeln!(
                source,
                "        pub const fn new() -> Self {{ Self({}) }}",
                control.constructor
            )?;
            match control.accessor {
                "value" => writeln!(
                    source,
                    "        pub const fn value(&self) -> u16 {{ self.0.value() }}"
                )?,
                "is_on" => writeln!(
                    source,
                    "        pub const fn is_on(&self) -> bool {{ self.0.is_on() }}"
                )?,
                "display" => writeln!(
                    source,
                    "        pub fn value(&self) -> &[u8] {{ self.0.value() }}"
                )?,
                _ => unreachable!(),
            }
            source.push_str("    }\n");
            writeln!(source, "    impl Default for {} {{", control.name)?;
            source.push_str("        fn default() -> Self { Self::new() }\n    }\n");
            writeln!(source, "    impl Control for {} {{", control.name)?;
            source.push_str(
                "        fn accept(&mut self, packet: Packet<'_>) { self.0.accept(packet); }\n",
            );
            source.push_str("        fn commit(&mut self) -> bool { self.0.commit() }\n    }\n\n");
        }
        source.push_str("}\n\n");
    }

    source.push_str("pub enum RegisteredControl<'a> {\n");
    for control in &controls {
        let callback = if control.enum_definition.is_some() {
            format!("{}::{}", control.group_module, control.callback)
        } else {
            control.callback.clone()
        };
        writeln!(
            source,
            "    {} {{ control: {}::{}, callback: {}<'a> }},",
            control.name, control.group_module, control.name, callback
        )?;
    }
    source.push_str("}\n\nimpl RegisteredControl<'_> {\n");
    source.push_str("    fn accept(&mut self, packet: Packet<'_>) {\n        match self {\n");
    for control in &controls {
        writeln!(
            source,
            "            Self::{} {{ control, .. }} => control.accept(packet),",
            control.name
        )?;
    }
    source.push_str("        }\n    }\n\n    fn commit(&mut self) {\n        match self {\n");
    for control in &controls {
        let callback_value = match control.accessor {
            "value" | "display" => "control.value()",
            "is_on" => "control.is_on()",
            _ => unreachable!(),
        };
        let commit_callback = if control.enum_definition.is_some() {
            control.commit_callback.replace(
                &format!("<{}", control.callback_argument),
                &format!("<{}::{}", control.group_module, control.callback_argument),
            )
        } else {
            control.commit_callback.clone()
        };
        writeln!(
            source,
            "            Self::{} {{ control, callback }} => if control.commit() {{ {} }},",
            control.name,
            if commit_callback.is_empty() {
                format!("callback({callback_value});")
            } else {
                commit_callback
            }
        )?;
    }
    source.push_str("        }\n    }\n}\n\n");

    writeln!(
        source,
        "pub struct {aircraft_name}Builder<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {{"
    )?;
    source.push_str("    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,\n    _packet_capacity: PhantomData<ProtocolParser<MAX_PACKET_DATA>>,\n}\n\n");
    writeln!(
        source,
        "impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {aircraft_name}Builder<'a, MAX_CONTROLS, MAX_PACKET_DATA> {{"
    )?;
    source.push_str("    pub fn new() -> Self { Self { controls: Vec::new(), _packet_capacity: PhantomData } }\n\n");
    for control in &controls {
        let callback_argument = if control.enum_definition.is_some() {
            format!("{}::{}", control.group_module, control.callback_argument)
        } else {
            control.callback_argument.clone()
        };
        writeln!(
            source,
            "    pub fn register_{}<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>",
            snake_case(&control.name)
        )?;
        writeln!(source, "    where F: FnMut({}) + 'a {{", callback_argument)?;
        writeln!(
            source,
            "        self.controls.push(RegisteredControl::{} {{ control: {}::{}::new(), callback }}).map_err(|_| \"aircraft control capacity exceeded\")?;",
            control.name, control.group_module, control.name
        )?;
        source.push_str("        Ok(self)\n    }\n\n");
    }
    writeln!(
        source,
        "    pub fn build(self) -> {aircraft_name}<'a, MAX_CONTROLS, MAX_PACKET_DATA> {{"
    )?;
    source.push_str("        ");
    writeln!(
        source,
        "{aircraft_name} {{ parser: ProtocolParser::new(), controls: self.controls, saw_frame: false }}"
    )?;
    source.push_str("    }\n}\n\n");
    writeln!(
        source,
        "impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> Default for {aircraft_name}Builder<'a, MAX_CONTROLS, MAX_PACKET_DATA> {{"
    )?;
    source.push_str("    fn default() -> Self { Self::new() }\n}\n\n");
    source.push_str("struct Dispatcher<'bus, 'callback, const MAX_CONTROLS: usize> {\n    controls: &'bus mut Vec<RegisteredControl<'callback>, MAX_CONTROLS>,\n    saw_frame: &'bus mut bool,\n}\n\n");
    source.push_str("impl<'bus, 'callback, const MAX_CONTROLS: usize> Sink for Dispatcher<'bus, 'callback, MAX_CONTROLS> {\n");
    source.push_str("    fn on_write(&mut self, packet: Packet<'_>) { for control in self.controls.iter_mut() { control.accept(packet); } }\n");
    source.push_str("    fn on_frame_sync(&mut self) { if *self.saw_frame { for control in self.controls.iter_mut() { control.commit(); } } else { *self.saw_frame = true; } }\n}\n\n");
    writeln!(
        source,
        "pub struct {aircraft_name}<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {{"
    )?;
    source.push_str("    parser: ProtocolParser<MAX_PACKET_DATA>,\n    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,\n    saw_frame: bool,\n}\n\n");
    writeln!(
        source,
        "impl<const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {aircraft_name}<'_, MAX_CONTROLS, MAX_PACKET_DATA> {{"
    )?;
    source.push_str("    pub fn feed(&mut self, bytes: &[u8]) -> Result<(), FeedError> {\n        let mut dispatcher = Dispatcher { controls: &mut self.controls, saw_frame: &mut self.saw_frame };\n        self.parser.feed(bytes, &mut dispatcher).map_err(FeedError::Parse)\n    }\n}\n");
    Ok(source)
}

fn generated_control(
    definition: &ControlDefinition,
    output: &OutputDefinition,
    output_index: usize,
) -> Result<Option<GeneratedControl>, Box<dyn std::error::Error>> {
    let name = if definition.outputs.len() == 1 {
        pascal_case(&definition.identifier)
    } else {
        let suffix = output
            .suffix
            .as_deref()
            .filter(|suffix| !suffix.is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| format!("_OUTPUT{}", output_index + 1));
        pascal_case(&format!("{}{}", definition.identifier, suffix))
    };
    let integer_base = match definition.control_type.as_str() {
        "selector"
        | "discrete_dial"
        | "electrically_held_switch"
        | "action"
        | "3Pos_2Command_Switch"
        | "3Pos_2Command_Switch_OpenClose"
        | "toggle_switch"
        | "emergency_parking_brake"
        | "mission_computer_switch" => "Selector",
        "analog_gauge" => "AnalogGauge",
        "led" => "Led",
        "limited_dial" => "LimitedDial",
        "analog_dial" | "variable_step_dial" => "AnalogDial",
        "fixed_step_dial" => "FixedStepDial",
        "metadata" | "frequency" => "Metadata",
        // Some CommonData entries use `display` for numeric metadata outputs.
        "display" => "Metadata",
        other => {
            return Err(format!(
                "unsupported control_type {other:?} for {}",
                definition.identifier
            )
            .into());
        }
    };
    let (base, callback, callback_argument, commit_callback, constructor, accessor) = match output
        .output_type
        .as_str()
    {
        "integer" => {
            let mask = output.mask.ok_or(format!(
                "integer output for {} has no mask",
                definition.identifier
            ))?;
            let shift = output.shift_by.ok_or(format!(
                "integer output for {} has no shift_by",
                definition.identifier
            ))?;
            let (callback, callback_argument, commit_callback) = if definition.control_type == "led"
            {
                ("LedCallback".to_string(), "bool".to_string(), String::new())
            } else if definition.control_type == "selector"
                && output.max_value.is_some_and(|max_value| max_value <= 10)
            {
                let enum_name = format!("{name}Value");
                (
                    format!("{enum_name}Callback"),
                    enum_name.clone(),
                    format!(
                        "if let Ok(value) = <{enum_name} as core::convert::TryFrom<u16>>::try_from(control.value()) {{ callback(value); }}"
                    ),
                )
            } else {
                (
                    "IntegerCallback".to_string(),
                    "u16".to_string(),
                    String::new(),
                )
            };
            (
                format!("{integer_base}<{}, {}, {}>", output.address, mask, shift),
                callback,
                callback_argument,
                commit_callback,
                format!("{integer_base}::new()"),
                if definition.control_type == "led" {
                    "is_on"
                } else {
                    "value"
                },
            )
        }
        "string" => {
            let length = output.max_length.ok_or(format!(
                "string output for {} has no max_length",
                definition.identifier
            ))?;
            (
                format!("Display<{}, {length}>", output.address),
                "DisplayCallback".to_string(),
                "&[u8]".to_string(),
                String::new(),
                "Display::new()".to_string(),
                "display",
            )
        }
        other => {
            return Err(format!(
                "unsupported output type {other:?} for {}",
                definition.identifier
            )
            .into());
        }
    };
    let mut description = String::new();
    if let Some(control_description) = &definition.description {
        description.push_str(control_description);
    }
    for input in &definition.inputs {
        if let Some(input_description) = &input.description {
            if !description.is_empty() {
                description.push('\n');
            }
            description.push_str("Input: ");
            description.push_str(input_description);
        }
    }
    if let Some(output_description) = &output.description {
        if !description.is_empty() {
            description.push('\n');
        }
        description.push_str("Output: ");
        description.push_str(output_description);
    }

    let enum_definition = if definition.control_type == "selector"
        && output.output_type == "integer"
        && output.max_value.is_some_and(|max_value| max_value <= 10)
    {
        let enum_name = format!("{name}Value");
        let max_value = output.max_value.unwrap();
        let mut definition_source = String::new();
        writeln!(
            definition_source,
            "#[derive(Clone, Copy, Debug, PartialEq, Eq)]"
        )?;
        writeln!(definition_source, "pub enum {enum_name} {{")?;
        for value in 0..=max_value {
            writeln!(definition_source, "    Select{},", value + 1)?;
        }
        writeln!(definition_source, "}}")?;
        writeln!(
            definition_source,
            "pub type {enum_name}Callback<'a> = &'a mut dyn FnMut({enum_name});"
        )?;
        writeln!(
            definition_source,
            "impl core::convert::TryFrom<u16> for {enum_name} {{"
        )?;
        writeln!(definition_source, "    type Error = u16;")?;
        writeln!(definition_source)?;
        writeln!(
            definition_source,
            "    fn try_from(value: u16) -> Result<Self, Self::Error> {{"
        )?;
        writeln!(definition_source, "        match value {{")?;
        for value in 0..=max_value {
            writeln!(
                definition_source,
                "            {value} => Ok(Self::Select{}),",
                value + 1
            )?;
        }
        writeln!(definition_source, "            value => Err(value),")?;
        writeln!(definition_source, "        }}")?;
        writeln!(definition_source, "    }}")?;
        writeln!(definition_source, "}}")?;
        Some(definition_source)
    } else {
        None
    };

    Ok(Some(GeneratedControl {
        name,
        group_module: String::new(),
        description,
        base,
        callback,
        callback_argument,
        commit_callback,
        constructor,
        accessor,
        enum_definition,
    }))
}

fn pascal_case(value: &str) -> String {
    let mut result: String = value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    let mut result = first.to_uppercase().collect::<String>();
                    result.extend(chars.map(|character| character.to_ascii_lowercase()));
                    result
                }
                None => String::new(),
            }
        })
        .collect();
    if result
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit())
    {
        result.insert_str(0, "Control");
    }
    result
}

fn module_case(value: &str) -> String {
    let mut result: String = value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .map(|character| character.to_ascii_lowercase())
        .collect();
    if result
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit())
    {
        result.insert(0, '_');
    }
    result
}

fn snake_case(value: &str) -> String {
    let mut result = String::new();
    for (index, character) in value.chars().enumerate() {
        if character.is_ascii_alphanumeric() {
            if character.is_ascii_uppercase() && index != 0 {
                result.push('_');
            }
            result.push(character.to_ascii_lowercase());
        } else if !result.ends_with('_') {
            result.push('_');
        }
    }
    result.trim_matches('_').to_string()
}

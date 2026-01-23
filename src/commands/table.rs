use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{
    LabeledError, ListStream, PipelineData, Record, ShellError, Signals, Signature, Span,
    SyntaxShape, Type, Value,
};

use crate::LdapPlugin;

pub struct Table;

impl PluginCommand for Table {
    type Plugin = LdapPlugin;

    fn name(&self) -> &str {
        "ldap table"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let args = call.rest::<String>(0)?;
        let attr_getters: Vec<_> = args
            .into_iter()
            .map(parse_attr_getter)
            .collect::<Result<Vec<_>, LabeledError>>()?;
        match input {
            PipelineData::Value(value, _) => Ok(handle_value(value, attr_getters)?),
            PipelineData::ListStream(list_stream, _) => {
                Ok(handle_list_stream(list_stream, attr_getters))
            }
            _ => Err(LabeledError::new("Unsupported input type")),
        }
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(PluginCommand::name(self))
            .input_output_types(vec![(
                Type::List(Box::new(Type::Record(Box::new([
                    (String::from("dn"), Type::String),
                    (String::from("attrs"), Type::record()),
                    (String::from("bin_attrs"), Type::record()),
                ])))),
                Type::list(Type::record()),
            )])
            .rest(
                "spec",
                SyntaxShape::String,
                "dn, attribute name ('+' suffix for multivalue)",
            )
    }

    fn description(&self) -> &str {
        "Transform an LDAP response into a flatter table"
    }
}

enum AttrGetter {
    Dn,
    Single(String),
    Multivalue(String),
}

fn parse_attr_getter(arg: String) -> Result<AttrGetter, LabeledError> {
    if arg == "dn" {
        Ok(AttrGetter::Dn)
    } else if arg.ends_with("+") {
        Ok(AttrGetter::Multivalue(
            arg.trim_end_matches('+').to_string(),
        ))
    } else {
        Ok(AttrGetter::Single(arg))
    }
}

fn reshape_record(record: &Record, attr_getters: &[AttrGetter]) -> Result<Value, ShellError> {
    let mut new_record = Record::new();
    let dn = record
        .get("dn")
        .ok_or_else(|| LabeledError::new("DN not found").into())
        .and_then(|dn| dn.as_str().map(|s| s.to_string()))?;
    let attrs = record
        .get("attrs")
        .ok_or_else(|| LabeledError::new("Attributes not found").into())
        .and_then(|attrs| attrs.as_record())?;

    for attr_getter in attr_getters {
        match attr_getter {
            AttrGetter::Dn => {
                let value = Value::string(dn.clone(), Span::unknown());
                new_record.insert("dn", value);
            }
            AttrGetter::Single(attr) => {
                let multivalue = attrs
                    .get(attr)
                    .cloned()
                    .unwrap_or_else(|| Value::list(vec![], Span::unknown()));
                let single = multivalue
                    .as_list()?
                    .first()
                    .cloned()
                    .unwrap_or_else(|| Value::string("".to_string(), Span::unknown()));
                new_record.insert(attr, single);
            }
            AttrGetter::Multivalue(attr) => {
                let multivalue = attrs
                    .get(attr)
                    .cloned()
                    .unwrap_or_else(|| Value::list(vec![], Span::unknown()));
                new_record.insert(attr, multivalue);
            }
        }
    }
    Ok(Value::record(new_record, Span::unknown()))
}

fn handle_value(value: Value, attr_getters: Vec<AttrGetter>) -> Result<PipelineData, LabeledError> {
    let list = value.as_list()?;
    let records = list
        .iter()
        .map(|record_value| {
            if let Ok(record) = record_value.as_record() {
                reshape_record(record, &attr_getters)
            } else {
                Err(LabeledError::new("Unsupported input type").into())
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(PipelineData::value(
        Value::list(records, Span::unknown()),
        None,
    ))
}

fn handle_list_stream(list_stream: ListStream, attr_getters: Vec<AttrGetter>) -> PipelineData {
    PipelineData::ListStream(
        ListStream::new(
            list_stream.into_iter().map(move |value| {
                if let Ok(record) = value.as_record() {
                    match reshape_record(record, &attr_getters) {
                        Ok(record) => record,
                        Err(err) => Value::error(err, Span::unknown()),
                    }
                } else {
                    Value::error(
                        LabeledError::new("Unsupported input type").into(),
                        Span::unknown(),
                    )
                }
            }),
            Span::unknown(),
            // TODO: Handle signals properly
            Signals::empty(),
        ),
        None,
    )
}

use nu_plugin::{EngineInterface, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Type};

use super::helper;
use super::opts::parse_opts;
use crate::LdapPlugin;

pub struct Search;

impl PluginCommand for Search {
    type Plugin = LdapPlugin;

    fn name(&self) -> &str {
        "ldap search"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let opts = parse_opts(call)?;

        todo!()
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(PluginCommand::name(self))
            .input_output_types(vec![(
                Type::Nothing,
                Type::List(
                    Box::new(Type::Any), // Replace with more detailed type
                ),
            )])
            .optional("uri", SyntaxShape::String, "URI of LDAP server")
            .optional("binddn", SyntaxShape::String, "Bind DN for authentication")
            .optional(
                "password",
                SyntaxShape::String,
                "Password for simple authentication",
            )
            .optional("basedn", SyntaxShape::String, "LDAP search base DN")
            .optional("scope", SyntaxShape::String, "LDAP search scope")
            .optional("size-limit", SyntaxShape::Number, "LDAP search size limit")
            .optional("time-limit", SyntaxShape::Number, "LDAP search time limit")
            .optional(
                "deref-aliases",
                SyntaxShape::String,
                "LDAP dereference aliases (never, search, find, always)",
            )
            .switch("typesonly", "Only return attribute names", None)
            .required("filter", SyntaxShape::String, "LDAP search filter")
            .rest("attributes", SyntaxShape::String, "LDAP attribute to fetch")
            .category(Category::Network)
    }

    fn description(&self) -> &str {
        "Searches an LDAP directory."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["ldap", "search", "directory"]
    }

    fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        vec![nu_protocol::Example {
            description: "Search for users in the LDAP directory",
            example: "ldap search '(objectClass=person)'",
            result: None,
        }]
    }
}

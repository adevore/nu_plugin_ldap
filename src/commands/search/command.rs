use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Type};

use super::search_impl::search_impl;
use crate::LdapPlugin;

pub struct Search;

impl PluginCommand for Search {
    type Plugin = LdapPlugin;

    fn name(&self) -> &str {
        "ldap search"
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        plugin
            .main_runtime
            .block_on(async move { search_impl(plugin, engine, call, input).await })
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(PluginCommand::name(self))
            .input_output_types(vec![(
                Type::Nothing,
                Type::List(
                    Box::new(Type::Any), // Replace with more detailed type
                ),
            )])
            .named("uri", SyntaxShape::String, "URI of LDAP server", None)
            .named(
                "binddn",
                SyntaxShape::String,
                "Bind DN for authentication",
                None,
            )
            .named(
                "password",
                SyntaxShape::String,
                "Password for simple authentication",
                None,
            )
            .named("basedn", SyntaxShape::String, "LDAP search base DN", None)
            .named("scope", SyntaxShape::String, "LDAP search scope", None)
            .named(
                "size-limit",
                SyntaxShape::Number,
                "LDAP search size limit",
                None,
            )
            .named(
                "time-limit",
                SyntaxShape::Number,
                "LDAP search time limit",
                None,
            )
            .named(
                "deref-aliases",
                SyntaxShape::String,
                "LDAP dereference aliases (never, search, find, always)",
                None,
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
            example: "ldap search '(uid=user)'",
            result: None,
        }]
    }
}

use zbus_xml::{ArgDirection, Method as XmlMethod, Node};

use crate::Method;

/// A snapshot of advertised methods. Missing and signature-incompatible methods
/// are unsupported; a call failure remains a transport/daemon error.
#[derive(Debug)]
pub struct Introspection(Node<'static>);

impl Introspection {
    pub fn parse(xml: &str) -> Result<Self, zbus_xml::Error> {
        Ok(Self(Node::try_from(xml)?.into_owned()))
    }

    pub fn has_method(&self, interface: &str, method: &str, input: &str, output: &str) -> bool {
        self.0
            .interfaces()
            .iter()
            .filter(|candidate| candidate.name().as_str() == interface)
            .flat_map(|candidate| candidate.methods())
            .filter(|candidate| candidate.name().as_str() == method)
            .any(|candidate| signatures(candidate) == (input.to_owned(), output.to_owned()))
    }

    pub fn supports<I, O>(&self, method: &Method<I, O>) -> bool {
        let info = method.info();
        self.has_method(info.interface, info.name, info.input, info.output)
    }
}

/// The concatenated input and output signatures. An argument without a
/// direction is an input, as the D-Bus specification requires for methods.
fn signatures(method: &XmlMethod<'_>) -> (String, String) {
    let mut input = String::new();
    let mut output = String::new();
    for arg in method.args() {
        let target = if arg.direction() == Some(ArgDirection::Out) {
            &mut output
        } else {
            &mut input
        };
        target.push_str(&arg.ty().to_string());
    }
    (input, output)
}

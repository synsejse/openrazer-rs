use zbus_xml::{ArgDirection, Node};

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
            .filter(|i| i.name().as_str() == interface)
            .any(|i| {
                i.methods()
                    .filter(|m| m.name().as_str() == method)
                    .any(|m| {
                        let mut ins = String::new();
                        let mut outs = String::new();
                        for arg in m.args() {
                            let target = if arg.direction() == Some(ArgDirection::Out) {
                                &mut outs
                            } else {
                                &mut ins
                            };
                            target.push_str(&arg.ty().to_string());
                        }
                        ins == input && outs == output
                    })
            })
    }
    pub fn supports<I, O>(&self, method: &crate::Method<I, O>) -> bool {
        let m = method.info();
        self.has_method(m.interface, m.name, m.input, m.output)
    }
}

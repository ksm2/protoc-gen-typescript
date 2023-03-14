use crate::printer::{Block, Module};
use protobuf::descriptor::{MethodDescriptorProto, ServiceDescriptorProto};
use protobuf::plugin::code_generator_response::File;
use std::borrow::Cow;
use std::collections::BTreeSet;

pub fn service(svc: &ServiceDescriptorProto) -> File {
    let name = svc.name();
    let mut module = Module::new(format!("{name}.ts"));

    let imports = service_imports(svc);
    for import in imports {
        module.import(&[&import]).from(&format!("./{import}"));
    }
    module.import(&["GrpcService"]).from("./types");
    module.blank();

    let mut class = module.class(name);

    class
        .property("service")
        .private()
        .readonly()
        .typed("GrpcService")
        .end();
    class.blank();

    let mut constr = class.constructor(&[("service", "GrpcService")]);
    constr.call("this.service = service;");
    constr.end();

    for method in &svc.method {
        let method_name = method.name();
        let req_type = &method.input_type()[1..];
        let res_type = &method.output_type()[1..];

        let param_type = if method.client_streaming() {
            format!("AsyncIterable<{req_type}>")
        } else {
            req_type.to_string()
        };
        let return_type = if method.server_streaming() {
            format!("AsyncIterable<{res_type}>")
        } else {
            format!("Promise<{res_type}>")
        };

        class.blank();
        let mut m = class.method(
            first_to_lower(method_name).as_ref(),
            &[("req", &param_type), ("sig?", "AbortSignal")],
            &return_type,
        );

        let method_type = method_type(method);
        m.call(&format!(
            "return this.service.{method_type}(\"{method_name}\", req, {res_type}, sig);"
        ));

        m.end();
    }
    class.end();

    module.into()
}

fn service_imports(svc: &ServiceDescriptorProto) -> BTreeSet<String> {
    svc.method
        .iter()
        .flat_map(|m| [m.input_type(), m.output_type()])
        .map(|str| &str[1..])
        .map(|str| str.to_string())
        .collect()
}

fn method_type(method: &MethodDescriptorProto) -> &'static str {
    let cs = method.client_streaming();
    let ss = method.server_streaming();
    if cs && ss {
        "bidiStreaming"
    } else if cs {
        "clientStreaming"
    } else if ss {
        "serverStreaming"
    } else {
        "unary"
    }
}

fn first_to_lower(str: &str) -> Cow<str> {
    if let Some(first) = str.chars().next() {
        if first.is_uppercase() {
            let mut out = first.to_lowercase().to_string();
            out.push_str(&str[1..]);
            return Cow::Owned(out);
        }
    }
    Cow::Borrowed(str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_to_lower() {
        assert_eq!(Cow::Borrowed(""), first_to_lower(""));
        assert_eq!(Cow::Borrowed("a"), first_to_lower("a"));
        assert_eq!(Cow::Borrowed("abc"), first_to_lower("abc"));

        assert_eq!(
            Cow::<str>::Owned(String::from("abc")),
            first_to_lower("Abc")
        );
    }
}

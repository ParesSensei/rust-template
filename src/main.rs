use handlebars::{handlebars_helper,Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;


struct DoubleNumber;

impl HelperDef for DoubleNumber {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc handlebars::Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult
    {
        let param = h.param(0).unwrap();
        let number = param.value().as_i64().unwrap();
        out.write(&format!("{}", number * 2))?;
        Ok(())
    }
}

#[derive(Serialize)]
struct Address {
    street: String,
    city: String,
}

#[derive(Serialize)]
struct Person {
    first_name: String,
    last_name: String,
    hobbies: Vec<String>,
    addresses: Vec<Address>,
}

#[derive(Serialize)]
struct Data {
    person: Person,
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_handlebars() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_string("hello", "Hello, {{name}}!")
        .unwrap();
    handlebars
        .register_template_string("bye", "Bye, {{name}}")
        .unwrap();

    let mut data = HashMap::new();
    data.insert("name", "ekotaro");

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered, "Hello, ekotaro!");

    let rendered = handlebars.render("bye", &data).unwrap(); //
    assert_eq!(rendered, "Bye, ekotaro");
}

#[test]
fn test_nested_variable() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_string("hello", "Hello, {{person.first_name}} {{person.last_name}}")
        .unwrap();

    let mut data = HashMap::new();

    let mut person = HashMap::new();
    person.insert("first_name", "ekotaro");
    person.insert("last_name", "kuroniwa");

    data.insert("person", &person);

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered, "Hello, ekotaro kuroniwa");
}

#[test]
fn test_html_escape() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_string("hello", "Hello, {{{name}}}!")
        .unwrap();

    let mut data = HashMap::new();
    data.insert("name", "<p>ekotaro</p>");

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered, "Hello, <p>ekotaro</p>!");
}

#[test]
fn test_template_file() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/hello.mustache")
        .unwrap();

    let mut data = HashMap::new();
    data.insert("name", "ekotaro");

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered, "Hello, ekotaro!");
}

#[test]
fn test_with() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/with-hello.mustache")
        .unwrap();

    let mut data = HashMap::new();

    let mut person = HashMap::new();
    person.insert("first_name", "ekotaro");
    person.insert("last_name", "kuroniwa");

    data.insert("person", &person);

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered.contains("<h1>Hello ekotaro kuroniwa</h1>"), true);
}

#[test]
fn test_serde() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/with-hello.mustache")
        .unwrap();

    let data = Data {
        person: Person {
            first_name: "ekotaro".to_string(),
            last_name: "kuroniwa".to_string(),
            hobbies: vec![],
            addresses: vec![],
        },
    };

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered.contains("<h1>Hello ekotaro kuroniwa</h1>"), true);
}

#[test]
fn test_serde_json() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/with-hello.mustache")
        .unwrap();

    let data = json!({
        "person": {
            "first_name": "ekotaro",
            "last_name": "kuroniwa"
        }
    });

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered.contains("<h1>Hello ekotaro kuroniwa</h1>"), true);
}

#[test]
fn test_if() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("layout/header", "templates/layout/header.mustache")
        .unwrap();
    handlebars
        .register_template_file("layout/footer", "templates/layout/footer.mustache")
        .unwrap();
    handlebars
        .register_template_file("hello", "templates/blog.mustache")
        .unwrap();

    let data = json!({
        "title": "belajar rust",
        "content": "belajar rust dengan baik",
        "footer": "Programmer zaman now",
    });

    let rendered = handlebars.render("hello", &data).unwrap();
    println!("{}", rendered);

    assert_eq!(rendered.contains("belajar rust"), true);
    assert_eq!(rendered.contains("belajar rust dengan baik"), true);
    assert_eq!(rendered.contains("Anonymous"), true);
    assert_eq!(rendered.contains("Programmer zaman now"), true);
}

#[test]
fn test_if2() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/blog.mustache")
        .unwrap();

    let data = json!({
       "title": "belajar rust",
        "content": "belajar rust dengan baik",
        "author": "ekotaro"
    });

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered.contains("belajar rust"), true);
    assert_eq!(rendered.contains("belajar rust dengan baik"), true);
    assert_eq!(rendered.contains("Anonymous"), false);
    assert_eq!(rendered.contains("ekotaro"), true);
}

#[test]
fn test_unless() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("footer", "templates/footer.mustache")
        .unwrap();

    let data = json!({});

    let rendered = handlebars.render("footer", &data).unwrap();
    assert_eq!(
        rendered.contains("this content does not contains footer"),
        true
    );
}

#[test]
fn test_unless2() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("footer", "templates/footer.mustache")
        .unwrap();

    let data = json!({
        "footer": "ekotaro"
    });

    let rendered = handlebars.render("footer", &data).unwrap();
    assert_eq!(
        rendered.contains("this content does not contains footer"),
        false
    );
}

#[test]
fn test_each() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("person", "templates/person.mustache")
        .unwrap();

    let data = Person {
        first_name: "ekotaro".to_string(),
        last_name: "kuroniwa".to_string(),
        hobbies: vec!["coding".to_string(), "decoding".to_string()],
        addresses: vec![],
    };

    let rendered = handlebars.render("person", &data).unwrap();
    assert_eq!(rendered.contains("ekotaro"), true);
    assert_eq!(rendered.contains("kuroniwa"), true);
    assert_eq!(rendered.contains("0 - coding"), true);
    assert_eq!(rendered.contains("1 - decoding"), true);
}

#[test]
fn test_each_object() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("person", "templates/person.mustache")
        .unwrap();

    let data = Person {
        first_name: "ekotaro".to_string(),
        last_name: "kuroniwa".to_string(),
        hobbies: vec!["coding".to_string(), "decoding".to_string()],
        addresses: vec![
            Address {
                street: "jl. Kampung 1".to_string(),
                city: "Jakarta".to_string(),
            },
            Address {
                street: "jl. Kampung 2".to_string(),
                city: "Bandung".to_string(),
            },
        ],
    };

    let rendered = handlebars.render("person", &data).unwrap();
    println!("{}", rendered);

    assert_eq!(rendered.contains("ekotaro"), true);
    assert_eq!(rendered.contains("kuroniwa"), true);
    assert_eq!(rendered.contains("0 - coding"), true);
    assert_eq!(rendered.contains("1 - decoding"), true);
    assert_eq!(rendered.contains("street - jl. Kampung 1"), true);
    assert_eq!(rendered.contains("city - Jakarta"), true);
    assert_eq!(rendered.contains("street - jl. Kampung 2"), true);
    assert_eq!(rendered.contains("city - Bandung"), true);
}

#[test]
fn test_helper() {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("double", Box::new(DoubleNumber));
    handlebars.register_template_string("helper", "Result: {{double value}}").unwrap();

    let data = json!({
        "value": 10
    });

    let rendered = handlebars.render("helper", &data).unwrap();
    assert_eq!(rendered.contains("Result: 20"), true);
}

handlebars_helper!(uppercase: |value: String| {
    value.to_uppercase()
});

#[test]
fn test_helper_macro() {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("uppercase", Box::new(uppercase));
    handlebars.register_template_string("helper", "Hello : {{uppercase name}}").unwrap();

    let data = json!({
        "name": "eko"
    });

    let rendered = handlebars.render("helper", &data).unwrap();
    assert_eq!(rendered.contains("Hello : EKO"), true);
}

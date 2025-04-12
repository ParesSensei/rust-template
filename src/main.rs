use std::collections::HashMap;
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct Person {
    first_name: String,
    last_name: String,
}

#[derive(Serialize)]
struct Data{
    person: Person
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_handlebars() {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_string("hello", "Hello, {{name}}!").unwrap();
    handlebars.register_template_string("bye", "Bye, {{name}}").unwrap();

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

    handlebars.register_template_string("hello", "Hello, {{person.first_name}} {{person.last_name}}").unwrap();

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

    handlebars.register_template_string("hello", "Hello, {{{name}}}!").unwrap();

    let mut data = HashMap::new();
    data.insert("name", "<p>ekotaro</p>");

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered, "Hello, <p>ekotaro</p>!");
}

#[test]
fn test_template_file() {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("hello", "templates/hello.mustache").unwrap();

    let mut data = HashMap::new();
    data.insert("name", "ekotaro");

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered, "Hello, ekotaro!");
}

#[test]
fn test_with() {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("hello", "templates/with-hello.mustache").unwrap();

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

    handlebars.register_template_file("hello", "templates/with-hello.mustache").unwrap();

    let data = Data{
      person: Person{
          first_name: "ekotaro".to_string(),
          last_name: "kuroniwa".to_string()
      }
    };

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered.contains("<h1>Hello ekotaro kuroniwa</h1>"), true);
}

#[test]
fn test_serde_json() {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("hello", "templates/with-hello.mustache").unwrap();

    let data = json!({
        "person": {
            "first_name": "ekotaro",
            "last_name": "kuroniwa"
        }
    });

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered.contains("<h1>Hello ekotaro kuroniwa</h1>"), true);
}
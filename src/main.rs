use std::collections::HashMap;
use handlebars::Handlebars;

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
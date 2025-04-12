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

#[test]
fn test_if() {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("hello", "templates/blog.mustache").unwrap();

    let data = json!({
       "title": "belajar rust",
        "content": "belajar rust dengan baik"
    });

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered.contains("belajar rust"), true);
    assert_eq!(rendered.contains("belajar rust dengan baik"), true);
    assert_eq!(rendered.contains("Anonymous"), true);
}

#[test]
fn test_if2() {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("hello", "templates/blog.mustache").unwrap();

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

    handlebars.register_template_file("footer", "templates/footer.mustache").unwrap();

    let data = json!({});

    let rendered = handlebars.render("footer", &data).unwrap();
    assert_eq!(rendered.contains("this content does not contains footer"), true);
}

#[test]
fn test_unless2() {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("footer", "templates/footer.mustache").unwrap();

    let data = json!({
        "footer": "ekotaro"
    });

    let rendered = handlebars.render("footer", &data).unwrap();
    assert_eq!(rendered.contains("this content does not contains footer"), false);
}
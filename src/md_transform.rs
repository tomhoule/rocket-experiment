use pulldown_cmark::{Event, Parser, Tag};
use pulldown_cmark::html::push_html;
use std::borrow::Cow;
use regex::Regex;

/// Renders a md string with custom link schemes to html, transforming the links into ones that
/// work on the app.
pub fn render(md: &str, edition_slug: &str) -> String {
    let tags = Parser::new(md);
    let mut rendered = String::new();
    let transformed = tags.map(|tag| {
        match tag {
            Event::Start(Tag::Link(target, inner)) => {
                Event::Start(Tag::Link(transform_link(edition_slug, target), inner))
            }
            other => other
        }
    });
    push_html(&mut rendered, transformed);
    rendered
}

lazy_static! {
    static ref ETHICS_URI: Regex = Regex::new(r#"ethics://(.*)"#).expect("re is valid");
}

pub fn transform_link<'a>(edition_slug: &str, original: Cow<'a, str>) -> Cow<'a, str> {
    {
        let captures = ETHICS_URI.captures(&original.as_ref());
        if let Some(captures) = captures {
            let mut transformed = format!(
                "/ethics/editions/{}/fragments/{}",
                edition_slug,
                &captures[1]
            );
            return Cow::from(transformed)
        }
    }
    original
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_link_works_as_expected() {
        let edition_slug = "slugs_are_delicious";
        let link = Cow::from("ethics://pt/1:p:33/demo");
        assert_eq!(
            transform_link(edition_slug, link).as_ref(),
            "/ethics/editions/slugs_are_delicious/fragments/pt/1:p:33/demo"
        );
    }

    #[test]
    fn md_transform_render_works() {
        let result = render(r##"
# Meow
        "##, "collector");
        let expected = r##"<h1>Meow</h1>
"##;
        assert_eq!(&result, expected);
    }
}

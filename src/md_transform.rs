use pulldown_cmark::{Event, Parser, Tag};
use pulldown_cmark::html::push_html;
use std::borrow::Cow;
use regex::Regex;
use percent_encoding::{percent_encode, PATH_SEGMENT_ENCODE_SET};

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
            let path = &captures[1];
            let mut transformed = format!(
                "/ethics/editions/{}/fragments/{}",
                edition_slug,
                percent_encode(path.as_bytes(), PATH_SEGMENT_ENCODE_SET)
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
            "/ethics/editions/slugs_are_delicious/fragments/pt%2F1:p:33%2Fdemo"
        );
    }

    #[test]
    fn md_transform_render_works() {
        let result = render(r##"
# Meow

This is [a link](ethics://pt/1:p:20:sch)
        "##, "collector");
        let expected = r##"<h1>Meow</h1>
<p>This is <a href="/ethics/editions/collector/fragments/pt%2F1:p:20:sch">a link</a></p>
"##;
        assert_eq!(&result, expected);
    }
}

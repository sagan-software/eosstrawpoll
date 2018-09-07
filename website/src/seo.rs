use stdweb::unstable::TryInto;
use stdweb::web::{document, Element, IElement, IParentNode};

pub fn set_title(title: &str) {
    let title = format!("{} - EOS Straw Poll", title);
    document().set_title(&title);
    set_attributes("meta[property='og:title']", "content", &title);
}

pub fn set_url(url: &str) {
    set_attributes("meta[property='og:url']", "content", url);
    set_attributes("link[rel='canonical']", "href", url);
}

pub fn set_description(description: &str) {
    set_attributes("meta[name='description']", "content", description);
    set_attributes("meta[property='og:description']", "content", description);
}

fn set_attributes(selector: &str, name: &str, value: &str) {
    if let Ok(nodes) = document().query_selector_all(selector) {
        for node in nodes.iter() {
            let el: Result<Element, _> = node.try_into();
            if let Ok(el) = el {
                el.set_attribute(name, value);
            }
        }
    }
}

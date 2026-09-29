use html_escape::encode_text;
use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

use crate::captcha::{get_pow, PoW};
use crate::utils::{append_query_prefix, get_domain_host};

const TRANSLATIONS_JSON: &str = include_str!("assets/translations.json");
const TEMPLATE_FILES: [(&str, &str); 2] = [
    ("captcha.html", include_str!("templates/captcha.html")),
    ("check.html", include_str!("templates/check.html")),
];

static TRANSLATIONS: LazyLock<RwLock<HashMap<String, HashMap<String, String>>>> =
    LazyLock::new(|| {
        RwLock::new(
            serde_json::from_str(TRANSLATIONS_JSON)
                .expect("embedded translations.json is valid"),
        )
    });
static TEMPLATES: LazyLock<RwLock<HashMap<String, String>>> = LazyLock::new(|| {
    RwLock::new(
        TEMPLATE_FILES
            .iter()
            .map(|(name, content)| (name.to_string(), content.to_string()))
            .collect(),
    )
});

fn get_template(template_name: &str) -> Option<String> {
    let templates = TEMPLATES.read().unwrap();
    templates.get(template_name).cloned()
}

fn translate_template(template: Option<String>, lang_code: &str) -> String {
    let mut template = match template {
        Some(t) => t,
        None => return String::new(),
    };

    if lang_code == "en" {
        return template;
    }

    let translations = TRANSLATIONS.read().unwrap();

    for (key, trans_map) in translations.iter() {
        if let Some(translation) = trans_map.get(lang_code) {
            template = template.replace(key, translation);
        }
    }

    template
}

fn render_template(template_name: &str, lang_code: &str, request_url: String) -> String {
    let template = get_template(template_name);
    let mut translated_template = translate_template(template, lang_code);

    translated_template =
        translated_template.replace("LANGUAGE", &encode_text(&lang_code));
    translated_template = translated_template.replace(
        "REQUESTURL",
        &encode_text(&append_query_prefix(&request_url)),
    );
    let domain = get_domain_host(request_url);
    translated_template =
        translated_template.replace("DOMAIN", &encode_text(domain.as_str()));

    translated_template
}

pub fn render_check(lang_code: &str, request_url: String, reason: String) -> String {
    let mut template = render_template("check.html", lang_code, request_url);

    let pow = PoW::new(get_pow(), 5);
    let (challenge, state) = pow.generate_challenge("127.0.0.1");
    template = template.replace("DIFFICULTY", "10");
    template = template.replace("POWCHALLENGE", &encode_text(&challenge));
    template = template.replace("POWSTATE", &encode_text(&state));
    template = template.replace("REASON", &encode_text(&reason));
    template
}

#[allow(dead_code)]
fn render_captcha(lang_code: &str, request_url: String) -> String {
    render_template("captcha.html", lang_code, request_url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_assets_load() {
        assert!(!TRANSLATIONS.read().unwrap().is_empty());
        assert!(get_template("check.html").is_some());
        assert!(get_template("captcha.html").is_some());
    }

    #[test]
    fn render_check_fills_placeholders() {
        let page = render_check("en", "http://example.com/a".into(), "TOR".into());
        assert!(!page.is_empty());
        for placeholder in ["POWCHALLENGE", "POWSTATE", "REASON", "REQUESTURL"] {
            assert!(!page.contains(placeholder), "{placeholder} left in page");
        }
    }
}

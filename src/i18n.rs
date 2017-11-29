use fluent::MessageContext;

pub struct I18nContexts {
    pub en: MessageContext<'static>,
    pub la: MessageContext<'static>,
}

impl I18nContexts {
    pub fn localize(&self, lang: &str, msg: &str) -> Option<String> {
        let context = match lang {
            "la" => &self.la,
            _ => &self.en,
        };
        context
            .get_message(msg)
            .and_then(|m| context.format(m, None))
    }
}

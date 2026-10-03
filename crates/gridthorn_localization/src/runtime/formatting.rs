use fluent_bundle::{FluentValue, types::FluentNumber};
use icu_decimal::{
    DecimalFormatter,
    input::Decimal,
    options::{DecimalFormatterOptions, GroupingStrategy},
};
use intl_memoizer::{Memoizable, concurrent::IntlLangMemoizer};
use unic_langid::LanguageIdentifier;
use writeable::Writeable;

pub(super) struct NumberFormatter(DecimalFormatter);

impl Memoizable for NumberFormatter {
    type Args = (bool,);
    type Error = String;

    fn construct(lang: LanguageIdentifier, (grouping,): Self::Args) -> Result<Self, Self::Error> {
        let locale: icu_locale_core::Locale = lang
            .to_string()
            .parse()
            .map_err(|error| format!("{error}"))?;
        let mut options = DecimalFormatterOptions::default();
        options.grouping_strategy = Some(if grouping {
            GroupingStrategy::Auto
        } else {
            GroupingStrategy::Never
        });
        DecimalFormatter::try_new(locale.into(), options)
            .map(Self)
            .map_err(|error| error.to_string())
    }
}

impl NumberFormatter {
    pub(super) fn format(&self, number: &FluentNumber) -> Result<String, String> {
        let decimal: Decimal = number
            .as_string()
            .parse()
            .map_err(|error| format!("{error}"))?;
        Ok(self.0.format(&decimal).write_to_string().into_owned())
    }
}

pub(super) fn format_value(value: &FluentValue<'_>, memoizer: &IntlLangMemoizer) -> Option<String> {
    let FluentValue::Number(number) = value else {
        return None;
    };
    memoizer
        .with_try_get::<NumberFormatter, _, _>((number.options.use_grouping,), |formatter| {
            formatter.format(number)
        })
        .ok()?
        .ok()
}

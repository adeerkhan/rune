//! A catalog with no locale, and a key index that does not fit in a span table.

use rune_i18n::{Catalog, I18n, LocaleInfo, Messages};

static LOCALES: [LocaleInfo; 1] = [LocaleInfo::new("en", "en", "English", false)];
static MESSAGES: [Messages; 1] = [Messages::new("Hello", &[0, 0, 0, 0, 5, 0, 0, 0])];
static CATALOG: Catalog = Catalog::new(&["greeting"], &[], &[], &LOCALES, &MESSAGES);

#[test]
fn a_catalog_needs_a_source_locale() {
    // The first locale is the source every key has a message in, so a catalog without one is
    // refused where it is built rather than panicking on the first lookup.
    let r = std::panic::catch_unwind(|| Catalog::new(&[], &[], &[], &[], &[]));
    assert!(r.is_err(), "an empty catalog is refused");
}

#[test]
fn a_catalog_answers_for_its_locale() {
    let i18n = I18n::new(&CATALOG);
    assert_eq!(i18n.info().id(), "en");
    assert_eq!(i18n.integer(1234), "1,234", "the number formats with the locale's grouping");
    assert_eq!(i18n.list(&["a", "b"]), "a and b", "the list uses the locale's conjunction");
    assert_eq!(i18n.plural_category(1), rune_i18n::PluralCategory::One);
}

#[test]
fn an_out_of_range_key_index_is_missing_rather_than_a_panic() {
    static M: Messages = Messages::new("Hello", &[0, 0, 0, 0, 5, 0, 0, 0]);
    assert_eq!(M.get(0), Some("Hello"));
    assert_eq!(M.get(1), None);
    // The span index is `key * 8`, which overflows for a large key.
    assert_eq!(M.get(usize::MAX / 4), None);
    assert_eq!(M.get(usize::MAX), None);
}

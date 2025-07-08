use color_eyre::Result;
pub mod raw_data;
pub mod translation_data;

pub fn fetch_data() -> Result<raw_data::Data> {
    todo!();
}

pub fn translate_data(
    orig_data: raw_data::Data,
    translation: translation_data::TranslationData,
) -> Result<raw_data::Data> {
    todo!();
}

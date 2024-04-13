use chrono::DateTime;
use pdfium_render::prelude::*;

use super::{Element, FontWeight, Page, Rect, TextElement};
use crate::{document::DocumentMeta, Result};

pub(super) fn extract_pdf_page(pdf_page: &PdfPage, page_num: usize) -> Result<Vec<Element>> {
    let pdf_page_text = pdf_page.text()?;
    let page = Page::builder()
        .page_num(page_num)
        .width(pdf_page.width().value)
        .height(pdf_page.height().value)
        .build();
    let elements = pdf_page
        .objects()
        .iter()
        .map(|object| extract_pdf_object(&page, &pdf_page_text, &object).unwrap())
        .flatten()
        .collect::<Vec<Element>>();

    Ok(elements)
}

pub(super) fn extract_pdf_object(
    page: &Page,
    pdf_page_text: &PdfPageText,
    object: &PdfPageObject,
) -> Result<Vec<Element>> {
    let mut results = vec![];
    match &object {
        PdfPageObject::Text(text_object) => {
            let pdf_bounds = text_object.bounds()?;
            let bounds = Rect::builder()
                .x1(pdf_bounds.left.value.round())
                .y1(pdf_bounds.top.value.round())
                .x2(pdf_bounds.right.value.round())
                .y2(pdf_bounds.bottom.value.round())
                .build();
            let font = text_object.font();
            let text: String = pdf_page_text.for_object(text_object);
            let font_weight = match font.weight()? {
                PdfFontWeight::Weight100 => FontWeight::Weight100,
                PdfFontWeight::Weight200 => FontWeight::Weight200,
                PdfFontWeight::Weight300 => FontWeight::Weight300,
                PdfFontWeight::Weight400Normal => FontWeight::Weight400Normal,
                PdfFontWeight::Weight500 => FontWeight::Weight500,
                PdfFontWeight::Weight600 => FontWeight::Weight600,
                PdfFontWeight::Weight700Bold => FontWeight::Weight700Bold,
                PdfFontWeight::Weight800 => FontWeight::Weight800,
                PdfFontWeight::Weight900 => FontWeight::Weight900,
                PdfFontWeight::Custom(x) => FontWeight::Custom(x),
            };
            let font_name = font.name();
            let font_size = text_object.scaled_font_size().value;

            let text = TextElement::builder()
                .text(text)
                .page(page.to_owned())
                .bounds(bounds)
                .font_name(font_name)
                .font_size(font_size)
                .font_weight(font_weight)
                .build();
            results.push(Element::Text(text));
        }
        PdfPageObject::XObjectForm(form_object) => {
            let result = form_object
                .iter()
                .map(|object| extract_pdf_object(page, pdf_page_text, &object).unwrap())
                .flatten()
                .collect::<Vec<Element>>();

            results.extend(result);
        }
        PdfPageObject::Image(_) => {}
        PdfPageObject::Path(_) => {}
        PdfPageObject::Shading(_) => {}
        PdfPageObject::Unsupported(_) => {}
    }

    Ok(results)
}

pub(crate) fn extract_meta(pdf_meta: &PdfMetadata) -> DocumentMeta {
    let title = match pdf_meta.get(PdfDocumentMetadataTagType::Title) {
        Some(tag) => tag.value().to_string(),
        None => String::new(),
    };

    let subject = match pdf_meta.get(PdfDocumentMetadataTagType::Subject) {
        Some(tag) => tag.value().to_string(),
        None => String::new(),
    };

    let keywords = match pdf_meta.get(PdfDocumentMetadataTagType::Keywords) {
        Some(tag) => tag.value().to_string(),
        None => String::new(),
    };

    let author = match pdf_meta.get(PdfDocumentMetadataTagType::Author) {
        Some(tag) => tag.value().to_string(),
        None => String::new(),
    };

    let creator = match pdf_meta.get(PdfDocumentMetadataTagType::Creator) {
        Some(tag) => tag.value().to_string(),
        None => String::new(),
    };

    let producer = match pdf_meta.get(PdfDocumentMetadataTagType::Producer) {
        Some(tag) => tag.value().to_string(),
        None => String::new(),
    };

    let creation_date_value = match pdf_meta.get(PdfDocumentMetadataTagType::CreationDate) {
        Some(tag) => tag.value().to_string(),
        None => String::new(),
    };

    let creation_date = match DateTime::parse_from_rfc3339(&creation_date_value) {
        Ok(dt) => Some(dt.timestamp_millis()),
        Err(_) => None,
    };

    let modification_date_value = match pdf_meta.get(PdfDocumentMetadataTagType::ModificationDate) {
        Some(tag) => tag.value().to_string(),
        None => String::new(),
    };

    let modification_date = match DateTime::parse_from_rfc3339(&modification_date_value) {
        Ok(dt) => Some(dt.timestamp_millis()),
        Err(_) => None,
    };

    DocumentMeta::builder()
        .title(title)
        .subject(Some(subject))
        .description(None)
        .keywords(Some(keywords))
        .author(Some(author))
        .creator(Some(creator))
        .producer(Some(producer))
        .language(None)
        .creation_date(creation_date)
        .modification_date(modification_date)
        .build()
}

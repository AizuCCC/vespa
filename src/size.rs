use printpdf::scale::Mm;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum Size {
    A4,
    A5,
    B4,
    B5,
}

impl Size {
    // return (width, heght)
    pub fn into_mm(&self) -> (printpdf::scale::Mm, printpdf::scale::Mm) {
        // see also https://raksul.com/magazine/column/a4size/
        match self {
            Size::A4 => (Mm(210.0), Mm(297.0)),
            Size::A5 => (Mm(148.0), Mm(210.0)),
            Size::B4 => (Mm(257.0), Mm(364.0)),
            Size::B5 => (Mm(182.0), Mm(257.0)),
        }
    }
}

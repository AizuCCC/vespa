use crate::toml_struct::*;
use anyhow::{Context, Result};
use std::collections::VecDeque;
use std::fs::File;
use std::ops::Range;

#[derive(Debug)]
pub struct ToC {
    pub title: String,
    pub front_author: String,
    pub back_author: String,
    pub editor: String,
    pub body: Vec<(Range<usize>, Option<String>, String)>, // range, title, author
}

impl ToC {
    pub fn new(title: String, front: String, back: String, editor: String) -> Self {
        Self {
            title: title,
            front_author: front,
            back_author: back,
            editor: editor,
            body: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum Page {
    FrontCover,
    BackCover,
    ToC,
    Colophon,
    BodyImg(File),
    // BodyTxt(File),
    Blank,
}

#[derive(Debug)]
pub struct Book {
    pub toc: ToC,
    pub colophon: Colophon,
    pub front_cover: File,
    pub back_cover: File,
    pub page: Vec<Page>,
}

fn collect_body(config: &Config, align: StartPage, parity: [usize; 2]) -> VecDeque<&Body> {
    let mut que: VecDeque<&Body> = config
        .body
        .iter()
        .filter(|a| a.start == align && a.files.len() % 2 == parity[0])
        .collect();
    que.append(
        &mut config
            .body
            .iter()
            .filter(|a| a.start == align && a.files.len() % 2 == parity[1])
            .collect(),
    );
    que
}

fn pagenation_(
    page: &mut Vec<Page>,
    pque: &mut VecDeque<&Body>,
    sque: &mut VecDeque<&Body>,
    // title, author, page num
) -> Result<(Option<String>, Option<String>, usize)> {
    if !pque.is_empty() {
        let p = pque.pop_front().unwrap();
        for p in &p.files {
            page.push(Page::BodyImg(
                File::open(p).with_context(|| format!("cannot open {:?}", p))?,
            ));
        }
        Ok((p.title.clone(), Some(p.author.clone()), p.files.len()))
    } else if !sque.is_empty() {
        let p = sque.pop_front().unwrap();
        for p in &p.files {
            page.push(Page::BodyImg(
                File::open(p).with_context(|| format!("cannot open {:?}", p))?,
            ));
        }
        Ok((p.title.clone(), Some(p.author.clone()), p.files.len()))
    } else {
        page.push(Page::Blank);
        Ok((None, None, 1))
    }
}

pub fn pagenation(config: &Config) -> Result<Book> {
    let mut toc = ToC::new(
        config.title.clone(),
        config.front.author.clone(),
        config.back.author.clone(),
        config.editor.clone(),
    );
    let front = File::open(&config.front.path)
        .with_context(|| format!("cannot open file {:?}", config.front.path))?;
    let back = File::open(&config.back.path)
        .with_context(|| format!("cannot open file {:?}", config.back.path))?;

    let mut right_que = collect_body(config, StartPage::Right, [0, 1]);
    let mut left_que = collect_body(config, StartPage::Left, [0, 1]);
    let mut auto_que = collect_body(config, StartPage::Auto, [1, 0]);

    let mut page = Vec::new();
    page.push(Page::FrontCover);
    page.push(Page::ToC);
    let mut page_idx = 2 as usize;
    while !right_que.is_empty() || !left_que.is_empty() || !auto_que.is_empty() {
        let (title, author, page_num);
        match page_idx % 2 {
            0 => (title, author, page_num) = pagenation_(&mut page, &mut left_que, &mut auto_que)?,
            1 => (title, author, page_num) = pagenation_(&mut page, &mut right_que, &mut auto_que)?,
            _ => panic!(),
        }
        if let Some(author) = author {
            toc.body
                .push((page_idx..(page_idx + page_num), title, author));
        }
        page_idx += page_num;
    }
    page.push(Page::Colophon);
    page.push(Page::BackCover);

    Ok(Book {
        toc: toc,
        colophon: config.colophon.clone(),
        front_cover: front,
        back_cover: back,
        page: page,
    })
}

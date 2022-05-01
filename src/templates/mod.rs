use askama::Template;
use crate::orm as orm;
use uuid::Uuid;

// docs: https://djc.github.io/askama/template_syntax.html

#[derive(Template)]
#[template(path="index.html")]
pub(crate) struct Index {
    pub(crate) laws: Vec<orm::LawData>,
    pub(crate) parties: Vec<orm::Party>,
    pub(crate) codexes: Vec<orm::Codex>,
}

#[derive(Template)]
#[template(path="laws.html")]
pub(crate) struct LawsPage {
    pub(crate) laws: Vec<orm::LawData>,
}

#[derive(Template)]
#[template(path="changeLaw.html")]
pub(crate) struct ChangeLawPage {
    pub(crate) id: Uuid,
}
use latex::{Document, DocumentClass, Element, List, ListKind, PreambleElement, Section};
use serde::{Deserialize, Serialize};

const PREAMBLE_ELEMENT: &'static str = "
\\titleformat{\\section}
{\\huge\\bfseries\\lowercase}
{}
{0em}
{}[\\titlerule]

\\titleformat{\\subsection}
{\\bfseries\\Large}
{}
{0em}
{}

\\titleformat{\\subsubsection}[runin]
{\\bfseries}
{}
{0em}
{}[---]

\\renewcommand{\\maketitle}{
		\\begin{center}
		{\\huge\\bfseries
		\\theauthor}

		\\vspace{.25em}
		jvsr@tutanota.com --- https://jvsr.dev
		\\end{center}
	}

";

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub mission: Option<String>,
    pub author: String,
    pub experiences: Vec<Subsection>,
    pub articles: Option<ListItems>,
    pub projects: Option<ListItems>,
    pub education: ListItems,
    pub skills: ListItems,
    pub keywords: Vec<String>,
    pub languages: ListItems,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListItems {
    pub description: String,
    pub list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subsection {
    pub title: String,
    pub description: Option<String>,
}

pub fn return_simple_section(msg: &str, name: &str) -> Section {
    let mut simple_section = Section::new(name);
    simple_section.push(msg);

    simple_section
}


pub fn return_list_in_subsections(items: Vec<Subsection>, section: &str) -> Section {
    let mut list_subsection_section = Section::new(section);

    for item in items {
        list_subsection_section.push(Element::UserDefined(format!(
            "\\subsection{{{}}}",
            item.title,
        )));

        if let Some(description) = &item.description {
            list_subsection_section.push(description.as_str());
        }
    }

    list_subsection_section
}

pub fn return_list_inline(items: &Vec<String>, section: &str) -> Section {
    let mut list_inline_section = Section::new(section);

    let mut keywords = String::new();

    for item in items {
        if keywords.len() == 0 {
            keywords.push_str("[");
            keywords.push_str(item);
            continue;
        }
        keywords.push_str(", ");
        keywords.push_str(item);
    }
    keywords.push_str("]");

    list_inline_section.push(keywords.as_str());
    list_inline_section
}

pub fn return_list(items: ListItems, section: &str) -> Section {
    let mut list_section = Section::new(section);
    let mut list = List::new(ListKind::Itemize);

    for item in items.list {
        list.push(item);
    }

    list_section.push(items.description.as_str());
    list_section.push(list);

    list_section
}

pub fn return_default_doc(title: &str, author: &str) -> Document {
    let mut doc = Document::new(DocumentClass::Article);
    doc.preamble
        .title(title)
        .author(author)
        .use_package("titlesec")
        .use_package("titlin")
        .push(PreambleElement::UserDefined(PREAMBLE_ELEMENT.to_string()));

    doc.push(Element::TitlePage);

    doc
}

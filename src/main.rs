use crate::tex::Data;
use latex::{Document, Element};
use std::{env, fs};
use tex::{
    return_default_doc, return_list, return_list_in_subsections, return_list_inline,
    return_simple_section,
};

mod tex;

fn create_document(data: Data) -> Document {
    let mut doc = return_default_doc("R\\'esume\\'e", &data.author);
    if let Some(mission) = data.mission {
        let mission_section = return_simple_section(&mission, "Mission");
        doc.push(Element::Section(mission_section));
    }

    let experiences_section = return_list_in_subsections(data.experiences, "Exeperiences");
    doc.push(Element::Section(experiences_section));

    if let Some(articles) = data.articles {
        let articles_section = return_list(articles, "Articles");
        doc.push(Element::Section(articles_section));
    }

    if let Some(projects) = data.projects {
        let projects_section = return_list(projects, "Projects");
        doc.push(Element::Section(projects_section));
    }

    let education_section = return_list(data.education, "Education");
    doc.push(Element::Section(education_section));

    let skills_section = return_list(data.skills, "Skills");
    doc.push(Element::Section(skills_section));

    let keywords_section = return_list_inline(&data.keywords, "Keywords");
    doc.push(Element::Section(keywords_section));

    let languages_section = return_list(data.languages, "Languages");
    doc.push(Element::Section(languages_section));

    doc
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let file = fs::read_to_string(&file_path);

    if let Err(_) = file {
        eprint!("Unable to find {} file", file_path);
        return;
    }

    let file = file.unwrap();
    let data: Data = match toml::from_str(&file) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Unable to parse data file, check out a example to fix the file");
            return;
        }
    };

    let doc = create_document(data);
    let rendered = latex::print(&doc).unwrap();
    println!("{}", rendered);
}

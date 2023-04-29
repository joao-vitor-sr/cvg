use latex::{Document, DocumentClass, Element, List, ListKind, PreambleElement, Section};

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

#[derive(Debug)]
struct Data {
    mission: Option<String>,
    author: String,
    experiences: Vec<Experience>,
    articles: ListItems,
    projects: ListItems,
    education: ListItems,
    skills: ListItems,
    keywords: Vec<String>,
    languages: ListItems
}

#[derive(Debug)]
struct ListItems {
    description: String,
    list: Vec<String>,
}

#[derive(Debug)]
struct Experience {
    title: String,
    description: Option<String>,
}

fn return_mision(mission: &str) -> Section {
    let mut mission_section = Section::new("Mission");
    mission_section.push(mission);

    mission_section
}

fn return_list_inline(items: &Vec<String>, section: &str) -> Section {
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

fn return_list(items: ListItems, section: &str) -> Section {
    let mut list_section = Section::new(section);
    let mut list = List::new(ListKind::Itemize);

    for item in items.list {
        list.push(item);
    }

    list_section.push(items.description.as_str());
    list_section.push(list);

    list_section
}

fn return_experiences(experiences: Vec<Experience>) -> Section {
    let mut experiences_section = Section::new("Experience");

    for experience in experiences {
        experiences_section.push(Element::UserDefined(format!(
            "\\subsection{{{}}}",
            experience.title,
        )));

        if let Some(description) = &experience.description {
            experiences_section.push(description.as_str());
        }
    }

    experiences_section
}

fn create_document(data: Data) -> Document {
    let mut doc = Document::new(DocumentClass::Article);
    doc.preamble
        .title("R\\'esume\\'e")
        .author(&data.author)
        .use_package("titlesec")
        .use_package("titlin")
        .push(PreambleElement::UserDefined(PREAMBLE_ELEMENT.to_string()));

    doc.push(Element::TitlePage);

    if let Some(mission) = data.mission {
        let mission_section = return_mision(&mission);
        doc.push(Element::Section(mission_section));
    }

    let experiences_section = return_experiences(data.experiences);
    doc.push(Element::Section(experiences_section));

    let articles_section = return_list(data.articles, "Articles");
    doc.push(Element::Section(articles_section));

    let projects_section = return_list(data.projects, "Projects");
    doc.push(Element::Section(projects_section));

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
    let data = Data {
        mission: Some(
            "Using mathematics to solve problems, research and learning through the
                         focused application of technology."
                .to_string(),
        ),
        author: "João Vitor Silveira Ribeiro".to_string(),
        experiences: vec![
            Experience {
                title: "E4 Sistemas".to_string(),
                description: Some(
                    "Engineer, Developer, (PHP, JS, Zend Framework, jQuery)
		October 2020 - today"
                        .to_string(),
                ),
            },
            Experience {
                title: "Zaap".to_string(),
                description: Some(
                    "Engineer, Developer, (PHP, JS, Zend Framework, jQuery)
		October 2020 - today"
                        .to_string(),
                ),
            },
        ],
        articles:
            ListItems { description: "".to_string(), list:
            vec![
            "Parasites vs Creators".to_string(),
            "Average Wage don't exist".to_string(),
            "Your security is a shit".to_string(),
            "Your security is a shit".to_string(),
            "Overview over linear algebra".to_string(),
            "We're fool at the random".to_string(),
        ],
            },
        projects: ListItems {
            description:
                "All projects are avaliable at the github (https://github.com/joao-vitor-sr)"
                    .to_string(),
            list: vec![
                "mms".to_string(),
                "scsm".to_string(),
                "cvg".to_string(),
                "bkm".to_string(),
                "tnt".to_string(),
            ],
        },
        education: ListItems {
            description: "".to_string(),
            list: vec![
                "Exact Sciences with profiency in mathematics (Universidade Federal do Paraná, 2023 - 2027)".to_string()
                ]
        },
        skills: ListItems { description: "".to_string(), list: vec![
		"Programming".to_string(),
		"Linux/UNIX Systems Architecture and Administration".to_string(),
		"Site Reliability Engineering (SRE)".to_string(),
		"Cloud Native Computing (DevOps)".to_string(),
		"System Software Design and Development".to_string(),
		"Scripted Automation and Orchestration".to_string(),
		"Network Protocol API Design and Development".to_string(),
		"Domain Language and Grammar Specification".to_string(),
		"Web and Proxy Server Setup and Administration".to_string(),
		"Structured Configuration Management and Exchange".to_string(),
		"Terminal User Interface (TUI) Specialization".to_string(),
		"Enterprise Domain and Data Modeling".to_string(),
		"Event-Driven Enterprise Business Patterns".to_string(),
		"Agile and Test Driven Development Practices".to_string(),
		"Multi-core Asynchronous Programming".to_string(),
		"Interprocess Communication Methods (IPC)".to_string(),
		"Core Dump and Root Cause Analysis".to_string(),
		"Cybersecurity Auditing and Testing".to_string(),
		"On-Prem Systems Hardware Administration".to_string(),
		"Supply Chain Integration".to_string(),
		"Writing and Publication".to_string(),
		"Mathematics".to_string(),
		"Physics".to_string(),
		"Chmistry".to_string(),
		"Writing".to_string()
        ] },
        keywords: vec!["Linux".to_string(), "Ubuntu".to_string(),
                      "Debian".to_string(), "Red Hat (CentOS)".to_string(),
                      "Kubernetes (k3s)".to_string(),
                      "Containers (Docker)".to_string(),
                      "Virtual Machines".to_string(), "VMWare".to_string(),
                      "Virtual Box".to_string(), "kk3os".to_string(),
                      "RancherOS".to_string(), "Raspberry Pi".to_string(),
                      "Arduino".to_string(), "UNIX".to_string(),
                      "Solaris".to_string(),"AIX".to_string(),
                      "Terminyl (TUI)".to_string(),
                      "Command Line (CLI)".to_string(),
                      "Secure Shell (SSH)".to_string(),
                      "File Systems (SAN/NAS)".to_string(),
                      "TCP/IP".to_string(), "UDP".to_string(),
                      "Sockets".to_string(), "HTTP".to_string(),
                      "SMTP".to_string(), "IMAP".to_string(),
                      "POP3".to_string(), "LDAP".to_string(),
                      "Samba".to_string(), "SFTP".to_string(),
                      "DNS".to_string(), "TLS".to_string(),
                      "Apache".to_string(), "Nginx".to_string(),
                      "Go (golang)".to_string(), "Go Templates".to_string(),
                      "Perl".to_string(), "Python".to_string(),
                      "Bash".to_string(), "POSIX Shell".to_string(),
                      "Expect (TCL)".to_string(),
                      "Fabric (FabFiles)".to_string(), "C".to_string(),
                      "gdb".to_string(), "gcc".to_string(), "strace".to_string(),
                      "HTML".to_string(), "CSS".to_string(),
                      "JavaScript".to_string(), "JAMStack".to_string(),
                      "Websockets".to_string(), "NodeJS/NPM".to_string(),
                      "PEGN".to_string(), "ABNF".to_string(),
                      "EBNF".to_string(), "SQL".to_string(),
                      "PostgreSQL".to_string(), "Redis".to_string(),
                      "UML".to_string(), "ERD".to_string(), "YAML".to_string(),
                      "JSON".to_string(), "XML".to_string(), "TOML".to_string(),
                      "Markdown".to_string(), "Pandoc".to_string(),
                      "GraphQL".to_string(), "REST".to_string(),
                      "gRPC".to_string(), "ProtoBuf".to_string(),
                      "curl".to_string(), "nmap".to_string(),
                      "Oauth2".to_string(), "Vim/Vi".to_string(), "VSCode".to_string(), "TMUX".to_string(), "screen".to_string(),
        "Lynx".to_string(), "w3m".to_string(), "Git".to_string(), "GitHub".to_string(),
        "GitLab".to_string(), "Gitea".to_string(),
        "DigitalOcean".to_string(), "AWS".to_string(), "IRC".to_string(), "Slack".to_string(),
        "Blog".to_string(), "Mathematics".to_string(), "Physics".to_string(),
        "Chemistry".to_string(),
        "Linear Algebra".to_string(), "Calculus".to_string(), "Statitics".to_string(),
        "Probability".to_string(), "Topology".to_string(), "Geometry".to_string(),
	"Differentil Geometry".to_string(), "Phylosophy".to_string()],
        languages: ListItems {
            description: "".to_string(),
            list: vec![
                "Brazilian Portuguese (native)".to_string(),
                "American English (spoken and written)".to_string()
            ]
        }
    };

    let doc = create_document(data);

    let rendered = latex::print(&doc).unwrap();
    println!("{}", rendered);
}

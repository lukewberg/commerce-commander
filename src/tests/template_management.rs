use crate::utils::proj_templating::ProjTemplating;

#[test]
fn add_template_repo() {
    ProjTemplating::add_template_repo(String::from("https://github.com/lukewberg/AcceleratorTemplates.git"));
}

#[test]
fn parse_repo_name() {
    let repo_name = ProjTemplating::parse_repo_name(&String::from("https://github.com/lukewberg/AcceleratorTemplates.git"));
    assert_eq!(repo_name, "AcceleratorTemplates")
}
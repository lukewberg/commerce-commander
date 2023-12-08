use crate::utils::proj_templating::ProjTemplating;

#[test]
fn add_template_repo() {
    ProjTemplating::add_template_repo(String::from("https://github.com/lukewberg/AcceleratorTemplates.git"));
}
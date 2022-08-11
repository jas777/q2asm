use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "hello.stpl")]
pub struct HelloTemplate {
    pub messages: Vec<String>
}

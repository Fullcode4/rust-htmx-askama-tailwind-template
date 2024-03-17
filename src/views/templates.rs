use askama::Template;


#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "dashboard/index.html")]
pub struct DashboardTemplate;

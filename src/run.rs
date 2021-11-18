use std::error::Error;
use std::path::Path;
use std::fs::File;

use keepass::{Database};
use web_view::*;
use simple_error::SimpleError;
use serde::{Serialize, Deserialize};

use crate::config;

fn html() -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang="">
  <head>
    <meta charset="utf-7">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width,initial-scale=1">
    <title>keepass-reader-ui</title>
    <style type="text/css">
      {styles}
    </style>
  </head>
  <body>
    <noscript>
      <strong>We're sorry but keepass-reader-ui doesn't work properly without JavaScript enabled. Please enable it to continue.</strong>
    </noscript>
    <div id="app"></div>
    <script type="text/javascript">
      {scripts}
    </script>
  </body>
</html>
"#,
        styles = include_str!("../ui/dist/css/app.css"),
        scripts = include_str!("../ui/dist/js/app.js")
    )
}

enum KeepassData {
    Filename(String)
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    if ! Path::new(&config.database_filename).exists() {
        return Err(
            Box::new(
                SimpleError::new(
                    format!("Database {} does not exist",
                            config.database_filename
                            ).as_str()
                    )
                )
            );
    }

    println!("Running with database: {}", config.database_filename);



    /*
    let entries = db.find_entries("");
    // println!("{:?}", db);
    println!("The database has {} entries:", entries.len());
    for entry in entries.iter() {
      println!("  - {} [{}/{}]", entry.title().unwrap(), entry.username().unwrap(), entry.password().unwrap());
    }
    */

    let content = Content::Html(html());
    // let content = Content::Url("http://localhost:8080/");
	
    let data = KeepassData::Filename(config.database_filename.clone());
    web_view::builder()
        .title("pKPR")
        .content(content)
        // .size(320, 480)
        .resizable(true)
        .debug(true)
        .user_data(data)
        .invoke_handler(|webview, arg| 
                        match invoke_handler(webview, arg) {
                            Ok(r) => Ok(r),
                            Err(e) => 
                                webview.eval(
                                    &format!(
                                        "window.get_app().handle_error('Internal error', '{:?}')",
                                        e)),
                        }
                        )
        .run()
        .unwrap();

    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Action {
    OnMounted,
    OpenDatabase { password: String }
}

#[derive(Serialize)]
struct Entry {
    pub title: String,
    pub username: String,
    pub password: String,
    pub notes: String,
    pub url: String,
}

fn invoke_handler(webview: &mut WebView<KeepassData>, arg: &str) -> Result<(), web_view::Error> {
    let action = serde_json::from_str(arg).map_err(|e| web_view::Error::Custom(Box::new(e)) )?;
    match action {
        Action::OnMounted => {
            let v = match webview.user_data() {
                KeepassData::Filename(v) => v,
            }.clone();
            webview.eval(&format!("window.get_app().set_database_filename('{}');", Path::new(v.as_str()).file_name().unwrap().to_str().unwrap()))?;
            Ok(())
        },
        Action::OpenDatabase { password } => {
            match webview.user_data() {
                KeepassData::Filename(v) => { 
                    let mut file = File::open(v).map_err(|e| web_view::Error::Custom(Box::new(e)))?; 
                    let key = Some(password.as_str()); 
                    let db = Database::open(&mut file, key, None).map_err(|e| web_view::Error::Custom(Box::new(e)))?;
                    let entries = keepass_database_node(&db.root).map_err(|e| web_view::Error::Custom(Box::new(SimpleError::new(format!("{:?}",e).as_str()))) )?;
                    let entry_list = entries.iter()
                        .map(|e| 
                                Entry { 
                                    title: e.get_title().unwrap_or("").to_string(), 
                                    username: e.get_username().unwrap_or("").to_string(), 
                                    password: e.get_password().unwrap_or("").to_string(), 
                                    notes: e.get("Notes").unwrap_or("").to_string(),
                                    url: e.get("URL").unwrap_or("").to_string()
                                }
                                )
                        .collect::<Vec::<Entry>>();
                    webview.eval(
                        &format!(
                            "window.get_app().set_entry_list({});", 
                            serde_json::to_string(&entry_list).map_err(|e| web_view::Error::Custom(Box::new(e)))?
                            )
                        )?;
                },
            };
            Ok(())
        },
    }
}

fn keepass_database_node(group: &keepass::Group) -> Result<Vec<& keepass::Entry>, Box<dyn Error>> {
    let mut r = Vec::new();
    for node in group.children.iter() {
        match node {
            keepass::Node::Group(g) => { r.append(&mut keepass_database_node(g)?); },
            keepass::Node::Entry(e) => { r.push(e); },
        }
    };
    Ok(r)
}

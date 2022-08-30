use mysql::*;
use mysql::prelude::*;
use crate::db::mysql::*;
use crate::dimensions::projectsettings::ProjectSettings;

impl MySQLDatabaseProxy {

  pub fn update_project_settings(&mut self, settings: &ProjectSettings) {
    let font_size = "7.5".to_string();
    let font_name = "Times New Roman".to_string();
    let res : Result::<Option::<u128>> = self.conn.exec_first("call updateProjectSettings(:proj,:bg,:goals,:scope,:picture,:font_size,:font)", params!{
      "proj" => &settings.name,
      "bg" => &settings.background,
      "goals" => &settings.strategic_goals,
      "scope" => &settings.scope,
      "picture" => &settings.rich_picture,
      "font_size" => &font_size,
      "font" => &font_name
    });
    match res {
      Ok(_r) => {},
      Err(err) => {panic!("MySQL error updating project settings {:?}",err);}
    }
 
    if let Err(err) = self.conn.query_first::<u128,&str>("call deleteDictionary()") {
      panic!("MySQL error deleting dictionary - {:?}",err);
    }

    for entry in &settings.naming_conventions {
      let res: Result::<Option<u128>> = self.conn.exec_first("call addDictionaryEntry(:e0,:e1)",params!{
        "e0" => entry.0,
        "e1" => entry.1
      });
      match res {
        Ok(_r) => {},
        Err(err) => {panic!("MySQL error adding naming convention - {:?}",err);}
      }
    }

    if let Err(err) = self.conn.query_first::<u128,&str>("call deleteContributors()") {
      panic!("MySQL error deleting contributors - {:?}",err);
    }

    for contributor in &settings.contributors {
      let res: Result::<Option<u128>> = self.conn.exec_first("call addContributorEntry(:e0,:e1,:e2,:e3)",params!{
        "e0" => &contributor.0,
        "e1" => &contributor.1,
        "e2" => &contributor.2,
        "e3" => &contributor.3
      });
      match res {
        Ok(_r) => {},
        Err(err) => {panic!("MySQL error adding contributor - {:?}",err);}
      }
    }

    if let Err(err) = self.conn.query_first::<u128,&str>("call deleteRevisions()") {
      panic!("MySQL error deleting revisions - {:?}",err);
    }

    for rev in &settings.revisions {
      let res: Result::<Option<u128>> = self.conn.exec_first("call addRevision(:e0,:e1,:e2)",params!{
        "e0" => &rev.0,
        "e1" => &rev.1,
        "e2" => &rev.2
      });
      match res {
        Ok(_r) => {},
        Err(err) => {panic!("MySQL error adding revision - {:?}",err);}
      }
    }

  }

  pub fn get_project_settings(&mut self) -> ProjectSettings {
    let mut ps = ProjectSettings::new(&"".to_string());
    let res : Result<Vec<(String,String)>> = self.conn.query("call getProjectSettings()");
    match res {
      Ok(rows) => {
        
        for row in rows {
          match row.0.as_str() {
            "Project Name" => {ps.name = row.1;},
            "Project Background" => {ps.background = row.1;},
            "Project Goals" => {ps.strategic_goals = row.1;},
            "Project Scope" => {ps.scope = row.1},
            "Rich Picture" => {ps.rich_picture = row.1},
            &_ => {}
          };
        }

        let nc_res : Result<Vec<(String,String)>> = self.conn.query("call getDictionary()");
        match nc_res {
          Ok(nc_rows) => {
            for nc in nc_rows {
              ps.naming_conventions.insert(nc.0,nc.1);
            }
          },
          Err(err) => {panic!("MySQL error getting naming conventions - {:?}",err);}
        };

        let conts_res : Result<Vec<(String,String,String,String)>> = self.conn.query("call getContributors()");
        match conts_res {
          Ok(conts_rows) => {
            for cont in conts_rows {
              ps.contributors.push((cont.0,cont.1,cont.2, cont.3));
            }
          },
          Err(err) => {panic!("MySQL error getting contributors - {:?}",err);}
        };

        let revs_res : Result<Vec<(String,String,String)>> = self.conn.query("call getRevisions()");
        match revs_res {
          Ok(revs_rows) => {
            for rev in revs_rows {
              ps.revisions.push((rev.0,rev.1,rev.2));
            }
          },
          Err(err) => {panic!("MySQL error getting revisions - {:?}",err);}
        };
      },
      Err(err) => {panic!("MySQL error getting settings - {:?}",err);}
    }
    return ps;
  }
}

use mysql::*;
use mysql::prelude::*;
use crate::db::mysql::*;
use crate::dimensions::environment::{Environment, CompositeEnvironments};

impl MySQLDatabaseProxy {

  pub fn add_environment(&mut self, env : &Environment) {
    self.commit_environment("add",env);
  }

  fn commit_environment(&mut self, commit_prefix: &str, env : &Environment) {

    let mut env_id = env.id;
    let sql_txt = format!("call {}Environment(:id,:name,:sc,:desc)",commit_prefix);
    if commit_prefix == "add" {
      env_id = self.new_id();
    }
    else {
      self.delete_environment_components(env_id);
    }

    let res : Result::<Option::<u128>> = self.conn.exec_first(sqlTxt, params!{
      "id" => env_id,
      "name" => &env.name,
      "sc" => &env.short_code,
      "desc" => &env.definition
    });
    match res {
      Ok(_r) => {},
      Err(err) => {panic!("MySQL error adding environment {:?}",err);}
    }
    
    if env.environments.len() > 0 {
      for ce in &env.environments.environments {
        self.add_composite_environment(env.id,&ce);
      }
      self.add_composite_environment_properties(env_id,&env.environments);
    }
  }

  fn delete_environment_components(&mut self, env_id: i128) {
    let res : Result::<Option::<u128>> = self.conn.exec_first("call deleteEnvironmentComponents(:id)", params!{
      "id" => env_id
    });
    match res {
      Ok(_r) => {},
      Err(err) => {panic!("MySQL error deleting environment components {:?}",err);}
    }
  }

  fn add_composite_environment(&mut self, env_id : i128, ce: &String) {
    let res : Result::<Option::<u128>> = self.conn.exec_first("call addCompositeEnvironment(:id,:c)", params!{
      "id" => env_id,
      "c" => ce
    });
    match res {
      Ok(_r) => {},
      Err(err) => {panic!("MySQL error adding composite environment {:?}",err);}
    }
  }

  fn add_composite_environment_properties(&mut self, env_id : i128, comp_env : &CompositeEnvironments) {
    let res : Result::<Option::<u128>> = self.conn.exec_first("call addCompositeEnvironmentProperties(:id,:dp,:oe)", params!{
      "id" => env_id,
      "dp" => comp_env.property.to_string(),
      "oe" => &comp_env.overriding_environment_name
    });
    match res {
      Ok(_r) => {},
      Err(err) => {panic!("MySQL error adding composite environment properties {:?}",err);}
    }
  }

  pub fn update_environment(&mut self, env: &Environment) {
    self.commit_environment("update",env);
  }

  pub fn delete_environment(&mut self, objt_id : i128) {
    self.delete_object(objt_id,"environment");
  }


}
use mysql::*;
use mysql::prelude::*;
use dotenv;
use std::process::Command;
use std::env;

pub struct MySQLDatabaseProxy {
  pub conn : PooledConn
}


impl MySQLDatabaseProxy {
  pub fn new(db_host: &String, db_port: &String, db_user: &String, db_passwd : &String, db_name : &String) -> MySQLDatabaseProxy {
    let url = format!("mysql://{}:{}@{}:{}/{}",db_user,db_passwd,db_host,db_port,db_name);
    match Pool::new(url.as_str()) {
      Result::Ok(p) => {
        match p.get_conn() {
          Result::Ok(c) => {
            return MySQLDatabaseProxy {conn: c};
          },
          Result::Err(err) => {
            panic!("{:?}",err);
          }
        };
      },
      Result::Err(err) => {
        panic!("{:?}",err);
      }
    };
  }

  pub fn ok(&mut self) -> bool {
    return self.conn.as_mut().ping();
  }


  pub fn new_id(&mut self) -> i128 {
    let res = self.conn.query_map("call newId()",|nid| nid);
    match res {
      Ok(r) => { return r[0];}
      Err(err) => { panic!("{:?}",err);}
    };
  }

  pub fn delete_object(&mut self, objt_id : i128, table_name: &str) {
    let sql_txt = format!("call delete_{}(:obj)",table_name);
    let res : Result::<Option::<u128>> = self.conn.exec_first(sql_txt, params!{
      "obj" => objt_id
    });
    match res {
      Ok(_r) => {},
      Err(err) => {println!("{:?}",err);}
    }
  }
}
  
pub fn initialise_db() -> MySQLDatabaseProxy {
  dotenv::dotenv().ok();
  Command::new(&env::var("RESET_SERVER").unwrap().as_str()).output().expect("Failed to initialise db");
  MySQLDatabaseProxy::new(&env::var("DB_HOST").unwrap(),&env::var("DB_PORT").unwrap(), &env::var("DB_USER").unwrap(), &env::var("DB_PASSWD").unwrap(), &env::var("TEST_DB").unwrap())
}

#[test]
pub fn test_database_ping() {
  let mut p = initialise_db();
  assert_eq!(p.ok(),true);
}
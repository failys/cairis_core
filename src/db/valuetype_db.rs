use mysql::*;
use mysql::prelude::*;
use crate::db::mysql::*;
use crate::dimensions::valuetype::ValueType;

impl MySQLDatabaseProxy {

  pub fn add_value_type(&mut self, vt : &ValueType) {
    let new_id = self.new_id();
    let res : Result::<Option::<u128>> = self.conn.exec_first("call addValueType(:id,:name,:desc,:type,:score,:rat)", params!{
      "id" => new_id,
      "name" => &vt.name,
      "desc" => &vt.description,
      "type" => &vt.vt_type,
      "score" => &vt.score,
      "rat" => &vt.rationale
    });
    match res {
      Ok(_r) => {},
      Err(err) => {println!("{:?}",err);}
    }
  }

  pub fn update_value_type(&mut self, vt : &ValueType) {
    let res : Result::<Option::<u128>> = self.conn.exec_first("call updateValueType(:id,:name,:desc,:type,:score,:rat)", params!{
      "id" => &vt.id,
      "name" => &vt.name,
      "desc" => &vt.description,
      "type" => &vt.vt_type,
      "score" => &vt.score,
      "rat" => &vt.rationale
    });
    match res {
      Ok(_r) => {},
      Err(err) => {println!("{:?}",err);}
    }
  }
  
  pub fn get_value_types(&mut self, dim_name : &String, env_name : &String) -> Vec<ValueType> {
    let res = self.conn.exec_map("call getCustomisableValues(:dim,:env)", params!{
      "dim" => dim_name,
      "env" => env_name
    },| (type_id, type_name, type_desc, type_value, type_rat) : (i128,String,String,i128,String) | {
      let mut vt = ValueType::new(&type_name,&type_desc,&"".to_string());
      vt.id = type_id;
      vt.score = type_value;
      vt.rationale = type_rat;
      vt.vt_type = dim_name.clone();
      return vt;
    });
    return res.unwrap();
  }

  pub fn delete_vulnerability_type(&mut self, objt_id : i128) {
    self.delete_object(objt_id,"vulnerability_type");
  }

}

#[test]
pub fn test_value_types() {
  let mut p = initialise_db();

  let mut no_vts = p.get_value_types(&"vulnerability_type".to_string(),&"".to_string());
  assert_eq!(no_vts.len(),0);

  let ivt = ValueType::new(&"AVT".to_string(),&"XXX".to_string(),&"vulnerability_type".to_string());
  p.add_value_type(&ivt);

  let ovts = p.get_value_types(&"vulnerability_type".to_string(),&"".to_string());
  let ovt = &ovts[0]; 
  assert_eq!(ivt.name,ovt.name);
  assert_eq!(ivt.description,ovt.description);
  assert_eq!(ivt.vt_type,ovt.vt_type);
  assert_eq!(-1,ovt.score);
  assert_eq!(ivt.rationale,ovt.rationale);
  assert_eq!(ivt.environment,ovt.environment);

  p.delete_vulnerability_type(ovt.id);
  no_vts = p.get_value_types(&"vulnerability_type".to_string(),&"".to_string());
  assert_eq!(no_vts.len(),0);
}
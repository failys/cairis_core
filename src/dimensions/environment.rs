use std::fmt;

#[derive(Clone,PartialEq,Debug)]
pub enum CompositeProperty {
  Override = 0,
  Maximise = 1
}

impl fmt::Display for CompositeProperty {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"{}",
      match self {
        CompositeProperty::Override => "Override".to_string(),
        CompositeProperty::Maximise => "Maximise".to_string()
      }
    )
  }
}

#[derive(Clone)]
pub struct CompositeEnvironments {
  pub environments : Vec<String>,
  pub property : CompositeProperty,
  pub overriding_environment_name : String,
}

impl CompositeEnvironments {
  pub fn new() -> CompositeEnvironments {
    CompositeEnvironments{ environments : Vec::<String>::new(), property : CompositeProperty::Maximise, overriding_environment_name : "".to_string()}
  }
  pub fn add(&mut self, new_env : &String) {
    self.environments.push(new_env.clone());
  }

  pub fn update_property(&mut self, prop_str : &String) {
    self.property = match prop_str.as_str() {
      "Override" => CompositeProperty::Override,
      "Maximise" => CompositeProperty::Maximise,
      &_ => CompositeProperty::Override
    };
  }

  pub fn len(&self) -> usize {
    self.environments.len()
  }
}

impl fmt::Display for CompositeEnvironments {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut env_str = "".to_string();
    for e in &self.environments {
      env_str.push_str(&e);
      env_str.push_str(" ");
    } 
    write!(f,"Environments: {}, Property: {}, Overriding Environment: {}",&env_str,&self.property.to_string(),self.overriding_environment_name)
  }
}

#[test]
fn test_new_composite_environments() {
  let ce = CompositeEnvironments::new();
  assert_eq!(ce.environments.len(),0);
  assert_eq!(ce.property,CompositeProperty::Maximise);
  assert_eq!(ce.overriding_environment_name,"".to_string());
}
#[test]
fn test_add_composite_environments() {
  let mut ce = CompositeEnvironments::new();
  ce.add(&"Foo".to_string());
  assert_eq!(ce.environments[0],"Foo".to_string());
}

#[test]
fn test_update_duplication_property() {
  let mut ce = CompositeEnvironments::new();
  ce.update_property(&"Override".to_string());
  assert_eq!(ce.property,CompositeProperty::Override);
}

#[derive(Clone)]
pub struct Environment {
  pub id : i128,
  pub name : String,
  pub short_code : String,
  pub definition : String,
  pub environments : CompositeEnvironments
}

impl Environment {
  pub fn new(env_name: &String, s_c : &String) -> Environment {
    Environment{ 
      id : -1,
      name : env_name.clone(), 
      short_code : s_c.clone(), 
      definition : "".to_string(), 
      environments : CompositeEnvironments::new()
    }
  }
}


impl fmt::Display for Environment {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"Name: {}, Short code: {}, Definition: {}, Environments: {}",self.name,self.short_code,self.definition,self.environments.to_string())
  }
}

#[test]
pub fn test_new_environment() {
  let e = Environment::new(&"Default".to_string(), &"DEF".to_string());
  assert_eq!(e.name,"Default".to_string());
  assert_eq!(e.short_code,"DEF".to_string());
  assert_eq!(e.definition,"".to_string());
}

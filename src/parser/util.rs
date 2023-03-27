use std::collections::HashMap;
use xml::attribute::OwnedAttribute;


pub fn parse_attributes(attributes: Vec<OwnedAttribute> ) -> HashMap<String, String> {
    attributes.iter().map(|x|
        (x.name.local_name.to_owned(), x.value.to_owned())
    ).collect()    
}
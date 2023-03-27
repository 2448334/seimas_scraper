use std::error::Error;
use std::io::Read;
use chrono::NaiveDate;
use xml::reader::{EventReader, XmlEvent};

use networking::request::*;
use crate::database::connect::establish_connection;
use crate::models::politicians::politician::{Politician, OfficeInsertable, Gender, DepartmentType};
use crate::networking;

use crate::parser::util::parse_attributes;

pub async fn get_politicians(parliament_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
    let link = format!("https://apps.lrs.lt/sip/p2b.ad_seimo_nariai?kadencijos_id={}", parliament_id);
    let xmlstring = url_request(&link).await?;
    parse_politicians(EventReader::from_str(&xmlstring));
    Ok(())
}

fn parse_politicians<R: Read> (eventreader: EventReader<R>) {
    let conn = &mut establish_connection();
    let mut parliament_id: Option<i32> = None;
    let mut politician: Option<Politician> = None;

    for e in eventreader {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "SeimoNarys" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        let id: i32 = keyvaluepairs["asmens_id"].parse().unwrap();
                        let parliament: i32 = parliament_id.unwrap();
                        let name: String = keyvaluepairs["vardas"].to_owned();
                        let surname: String = keyvaluepairs["pavardė"].to_owned();
                        let gender: Option<Gender> = if keyvaluepairs["lytis"] == "V" {
                            Some(Gender::M)
                        } else {
                            Some(Gender::F)
                        };
                        let from: Option<NaiveDate> = NaiveDate::parse_from_str(&keyvaluepairs["data_nuo"], "%Y-%m-%d").ok();
                        let to: Option<NaiveDate> = NaiveDate::parse_from_str(&keyvaluepairs["data_iki"], "%Y-%m-%d").ok();
                        let party: Option<String> = Some(keyvaluepairs["iškėlusi_partija"].to_owned());
                        let elected_type: Option<String> = Some(keyvaluepairs["išrinkimo_būdas"].to_owned());
                        let biography_link: Option<String> = keyvaluepairs.get("biografijos_nuoroda").and_then(|x| Some(x.to_owned()));
                        let term_count: Option<i32> = keyvaluepairs["kadencijų_skaičius"].parse().ok();

                        politician = Some(Politician{
                            id,
                            parliament,
                            name,
                            surname,
                            gender,
                            from,
                            to,
                            party,
                            elected_type,
                            biography_link,
                            term_count,
                            email: None,
                            phone: vec![],
                            website: None,
                            offices: vec![],
                        });
                    },
                    "Pareigos" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        let (department_id, department_type) = if keyvaluepairs.contains_key("padalinio_id") {
                            (keyvaluepairs["padalinio_id"].parse().ok(), Some(DepartmentType::Office))
                        } else {
                            (keyvaluepairs["parlamentinės_grupės_id"].parse().ok(),  Some(DepartmentType::Group))
                        };
                        
                        let department_name = if keyvaluepairs.contains_key("padalinio_pavadinimas") {
                            Some(keyvaluepairs["padalinio_pavadinimas"].to_owned())
                        } else {
                            Some(keyvaluepairs["parlamentinės_grupės_pavadinimas"].to_owned())
                        };

                        let duties = Some(keyvaluepairs["pareigos"].to_owned());
                        let from: Option<NaiveDate> = NaiveDate::parse_from_str(&keyvaluepairs["data_nuo"], "%Y-%m-%d").ok();
                        let to: Option<NaiveDate> = NaiveDate::parse_from_str(&keyvaluepairs["data_iki"], "%Y-%m-%d").ok();

                        let office = OfficeInsertable {
                            department_id,
                            department_name,
                            department_type,
                            duties,
                            from,
                            to,
                        };
                        let office = office.save(conn).unwrap().unwrap();
                        if let Some(unwrapped_politician) = &mut politician {
                            unwrapped_politician.offices.push(Some(office.id));
                        }
                    }
                    "Kontaktai" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        let contact_type = keyvaluepairs["rūšis"].to_owned();
                        let contact_value = keyvaluepairs["reikšmė"].to_owned();
                        match contact_type.as_str() {
                            "El. p." => {
                                if let Some(unwrapped_politician) = &mut politician {
                                    unwrapped_politician.email = Some(contact_value);
                                }
                            }
                            "Darbo telefonas" => {
                                if let Some(unwrapped_politician) = &mut politician {
                                    unwrapped_politician.phone.push(Some(contact_value));
                                }
                            }
                            "Asmeninė interneto svetainė" => {
                                if let Some(unwrapped_politician) = &mut politician {
                                    unwrapped_politician.website = Some(contact_value);
                                }
                            }
                            _ => {
        
                            }
                        }
                    }
                    "SeimoKadencija" => {
                        let keyvaluepairs = parse_attributes(attributes);
                        parliament_id = keyvaluepairs["kadencijos_id"].parse().ok();
                    }
                    _ => {

                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    "SeimoKadencija" => {
                        parliament_id = None;
                    },
                    "SeimoNarys" => {
                        if let Some(unwrapped_politician) = &mut politician {
                            unwrapped_politician.save(conn).unwrap();
                        }
                        politician = None;
                    },
                    _ => {

                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

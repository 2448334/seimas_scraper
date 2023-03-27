use std::path::Path;
use std::{error::Error, fs::File};

use std::io::Write;
use log::{info, error, debug};
use reqwest::Url;

use async_process::Command;

async fn get_document(doc_id: &str, filename: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let filepath = format!("documents/{}.txt", filename);
    if Path::new(&filepath).exists() {
        debug!("File exists: {}", filepath);
        return Ok(false)
    }
    let filepath = format!("documents/{}.odt", filename);
    info!("Downloading file: {}...", filepath);

    let link = format!("https://e-seimas.lrs.lt/rs/legalact/TAK/{}/format/OO3_ODT/", doc_id);

    let url = Url::parse(&link)?;
    let response = reqwest::get(url).await?;
    


    let docpath = Path::new("documents");
    if !docpath.exists() {
        std::fs::create_dir_all(docpath)?;
    }
    let mut file = File::create(filepath.clone())?;

    
    let bytes = response.bytes().await?;

    file.write_all(&bytes)
            .or(Err(format!("Error while writing to file")))?;
    
    match convert_odt_to_txt(&filepath).await {
        Ok(_) => {}
        Err(error) => {
            error!("Error converting odt to txt: {:?}", error);
        }
    }
    debug!("Downloaded file: {}", filepath);
    Ok(true)
}

pub async fn get_protocol_document(link: String, session_id: i32, meeting_num: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = Url::parse(&link)?;
    let doc_id = url
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap();
    let filename = format!("protocol_{}_{}_{}", session_id, meeting_num, doc_id);
    get_document(&doc_id, &filename).await?;
    Ok(())
}

pub async fn get_stenogram_document(link: String, session_id: i32, meeting_num: i32)-> Result<(), Box<dyn Error + Send + Sync>> {
    let url = Url::parse(&link)?;
    let doc_id = url
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap();
    let filename = format!("stenogram_{}_{}_{}", session_id, meeting_num, doc_id);
    if get_document(&doc_id, &filename).await? {
        let fullname = format!("documents/{}.txt", filename);
        debug!("Fixing stenogram file {}...", fullname);
        Command::new("sed").arg("-i").arg("s/­//g").arg(fullname).output().await?;
    }
    Ok(())
}


pub async fn convert_odt_to_txt(filename: &str) -> Result<(), Box<dyn Error>> {
    Command::new("libreoffice").arg("--convert-to").arg("txt").arg(filename).arg("--outdir").arg("documents").output().await?;
    Command::new("rm").arg(filename).output().await?;
    Ok(())
}


    use actix_web::{ 
        HttpServer, 
        App, 
        web, 
        Result, 
    };

    use std::path::PathBuf;
    use actix_files::NamedFile;

async fn index() -> Result<NamedFile>{

    let path: PathBuf = "./views/index.html".parse().unwrap();
    if !path.starts_with("./views").clone() {
        Err(NamedFile::open(path.clone()).unwrap()).expect("No File exist or empty path")
    }
    Ok(NamedFile::open(path.clone())?)
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    println!("Application started @ 127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
                .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

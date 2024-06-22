use clap::Parser;
use std::process::{Command};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[derive(Parser)]
#[command(name = "antlr_cli")]
#[command(about = "A CLI to run ANTLR parser and lexer", long_about = None)]
struct Cli {
    /// The grammar file to process
    #[arg(short, long)]
    grammar: String,

    /// The input file to lex
    #[arg(short, long)]
    input: String,
}

async fn tokenize(data: web::Json<String>) -> impl Responder {
    let input = data.into_inner();

   
    let cli = Cli::parse();

    // Compile the grammar using ANTLR
    let output = Command::new("antlr4")
    .arg(&cli.grammar)
    .output()
    .expect("failed to execute process");

    let tokenized_output = String::from_utf8_lossy(&output.stdout).to_string();

    HttpResponse::Ok().body(tokenized_output)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/tokenize", web::post().to(tokenize))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// fn main() -> Result<()> {
//     let cli = Cli::parse();

//     // Compile the grammar using ANTLR
//     let antlr_status = Command::new("antlr4")
//     .arg(&cli.grammar)
//     .status()?;

//     if !antlr_status.success() {
//         eprintln!("ANTLR grammar compilation failed.");
//         std::process::exit(1);
//     }

//     // // Compile the generated lexer and parser
//     // let javac_status = Command::new("javac")
//     //     .arg(format!("{}*.java", cli.grammar.split('.').next().unwrap()))
//     //     .status()?;

//     // if !javac_status.success() {
//     //     eprintln!("Compilation of generated Java files failed.");
//     //     std::process::exit(1);
//     // }

//     // // Run the lexer
//     // let lexer_class = format!("{}Lexer", cli.grammar.split('.').next().unwrap());
//     // let lexer_status = Command::new("java")
//     //     .arg(lexer_class)
//     //     .arg(&cli.input)
//     //     .status()?;

//     // if !lexer_status.success() {
//     //     eprintln!("Running the lexer failed.");
//     //     std::process::exit(1);
//     // }

//     Ok(())
// }

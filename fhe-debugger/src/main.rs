use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use petgraph::{
    dot::Dot,
    stable_graph::{EdgeReference, Edges, Neighbors, NodeIndex, StableGraph},
    visit::{EdgeRef, IntoNodeIdentifiers},
    Directed, Direction,
};
use sunscreen::{
    fhe_program,
    types::{bfv::Signed, Cipher},
    Compiler, Error, Runtime,
};
use sunscreen_compiler_common::{
    CompilationResult, Context, EdgeInfo, NodeInfo, Operation, Render,
};
use sunscreen_fhe_program::FheProgram;

#[fhe_program(scheme = "bfv")]
fn simple_multiply(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
    a * b
}

#[fhe_program(scheme = "bfv")]
fn simple_add(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
    a + b
}

#[get("/multiply")]
async fn multiply_handler() -> impl Responder {
    match process_multiply().await {
        Ok(result) => HttpResponse::Ok().body(format!("Result: {:?}", result)),
        Err(err) => {
            eprintln!("Error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/add")]
async fn add_handler() -> impl Responder {
    match process_add().await {
        Ok(result) => HttpResponse::Ok().body(format!("Result: {:?}", result)),
        Err(err) => {
            eprintln!("Error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/fhe")]
async fn fhe_handler() -> impl Responder {
    match process_fhe().await {
        Ok(result) => HttpResponse::Ok().body(format!("Result: {:?}", result)),
        Err(err) => {
            eprintln!("Error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn process_add() -> Result<Signed, Error> {
    let app = Compiler::new().fhe_program(simple_add).compile()?;

    let runtime = Runtime::new_fhe(app.params())?;

    let (public_key, private_key) = runtime.generate_keys()?;

    let a = runtime.encrypt(Signed::from(15), &public_key)?;
    let b = runtime.encrypt(Signed::from(5), &public_key)?;

    let results = runtime.run(
        app.get_fhe_program(simple_add).unwrap(),
        vec![a.clone(), b.clone()],
        &public_key,
    )?;
    let c: Signed = runtime.decrypt(&results[0], &private_key)?;

    Ok(c)
}

async fn process_multiply() -> Result<Signed, Error> {
    let app = Compiler::new().fhe_program(simple_multiply).compile()?;

    let runtime = Runtime::new_fhe(app.params())?;

    let (public_key, private_key) = runtime.generate_keys()?;

    let a = runtime.encrypt(Signed::from(15), &public_key)?;
    let b = runtime.encrypt(Signed::from(5), &public_key)?;

    let results = runtime.run(
        app.get_fhe_program(simple_multiply).unwrap(),
        vec![a.clone(), b.clone()],
        &public_key,
    )?;
    let c: Signed = runtime.decrypt(&results[0], &private_key)?;

    Ok(c)
}

async fn process_fhe() -> Result<FheProgram, Error> {
    let app = Compiler::new().fhe_program(simple_add).compile()?;

    let test = app.get_fhe_program(simple_add).unwrap().clone();
    let test2 = test.fhe_program_fn;

    Ok(test2)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(multiply_handler)
            .service(add_handler)
            .service(fhe_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

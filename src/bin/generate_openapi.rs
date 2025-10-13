use minotari::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let openapi_spec = ApiDoc::openapi().to_pretty_json().unwrap();
    let output_path = "openapi.json";
    std::fs::write(output_path, openapi_spec).expect("Failed to write OpenAPI spec to file");

    println!("✅ OpenAPI spec generated and written to {}", output_path);
}

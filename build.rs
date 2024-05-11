use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("CMAKE_aarch64_apple_darwin", "/opt/homebrew/bin/cmake");
    
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/services.proto"],   // Path to your proto file
            &["proto"],                // Directory where the proto file is located
        )?;
    Ok(())
}
pub fn tracing_setup() -> Result<(), anyhow::Error> {
    let tracer = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(tracer)?;
    Ok(())
}

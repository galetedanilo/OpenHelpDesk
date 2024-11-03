use OpenHeldDesk::startup::Application;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    Application::run().await;

    Ok(())
}

mod todos;

use std::error::Error;
use todos::TodoAppState;


fn main() -> Result<(), Box<dyn Error>> {
    let mut app = TodoAppState::init()?;
    app.run()?;
    Ok(())
}

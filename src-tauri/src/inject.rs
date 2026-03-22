use enigo::{Enigo, Keyboard, Settings};

pub fn type_text(text: &str) -> anyhow::Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    enigo.text(text)?;
    Ok(())
}

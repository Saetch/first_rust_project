use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe{
        MessageBoxA(None, "Text", "Caption", MB_OK);
      
    }  

    Ok(())
}
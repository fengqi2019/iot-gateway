wit_bindgen_rust::export!("./config/wit/wasi.wit");

pub struct Wasi;

impl wasi::Wasi for Wasi {
    fn wasi(_param: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        Ok(Vec::new())
    }
}

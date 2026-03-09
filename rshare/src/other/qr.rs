use image::Luma;
use local_ip_address::local_ip;
use qrcode;
use std::path::Path;

pub fn create_qr(port: u16, output_path: &Path) {
    let ip = local_ip().expect("No local ip");
    let url = format!("http://{}:{}/gallery", ip, port);

    let code = qrcode::QrCode::new(url).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save(output_path).unwrap();

    let string = code.render().light_color(' ').dark_color('#').build();
    println!("{}", string);
}

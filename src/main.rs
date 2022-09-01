use std::fs::File;
use std::io::Read;

fn main() {
    let mut vertex_file =
        File::open("assets/vertex_shader.glsl").expect("Error opening vertex_shader.glsl!");

    let mut file_data = String::new();
    vertex_file.read_to_string(&mut file_data).expect("Error reading vertex_shader.glsl!");

    println!("{}", file_data);
}

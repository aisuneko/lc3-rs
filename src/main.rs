mod key_queue;
mod lc3;
use lc3::LC3;
use lc3_image::read_image;

const DEFAULT_PROGRAM: &str = "./programs/obj/simpadd.obj";

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let default_program = DEFAULT_PROGRAM.to_string();
    let path = args.get(1).unwrap_or(&default_program);
    let (program, pc_start) = read_image(path).unwrap();
    let mut lc3 = LC3::new(&program, pc_start);
    lc3.execute();
}

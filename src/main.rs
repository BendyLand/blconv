mod image;
mod utils;

fn main() {
    let args = utils::get_cl_args();
    if args.len() > 2 {
        let err = image::convert_image(args[1].clone(), args[2].clone()).unwrap();
        if err != () {
            println!("There was an error converting your image: {:?}", err);
        }
    }
}

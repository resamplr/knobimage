extern crate image;

use std::io::Write;
use std::env;
use std::fs;
use image::{
    GenericImage,
    ImageBuffer
};

struct Dimension {
	width: u32,
	height: u32,
	final_height: u32
}

fn main() {

	// parse args
	for argument in env::args() {
	    println!("{}", argument);
	}

	// get vector of all files in working directory
	let paths: Vec<std::fs::DirEntry> = fs::read_dir("./").unwrap().into_iter().map(|x| x.unwrap()).collect();

	// check errors
	match paths.len() {
		0 => panic!("no images found"),
		1 => panic!("need more than one image"),
		_ => ()
	}	

	// let the user know what we're doing
	println!("Stitching together image {} to image {}.", paths.first().unwrap().path().display(), paths.last().unwrap().path().display());

	// figure out the size of our image.  Multiply it by the number of files in our vector to get the final height
	let test_img = image::open(paths.first().unwrap().path()).unwrap();
	let dimensions = Dimension {
		width: test_img.dimensions().0,
		height: test_img.dimensions().1,
		final_height: test_img.dimensions().1 * paths.len() as u32
	};
	
	println!("Final image size will be {}x{}", dimensions.width, dimensions.final_height);

	// crete our canvas
	let mut final_image: image::RgbaImage = ImageBuffer::new(dimensions.width, dimensions.final_height);
	// loop over every file (open individually, try not to kill ram) and append
	for (i, path) in paths.iter().enumerate() {
		// calculate top margin based on index
		let top = i as u32 * dimensions.height;
		// load our image
		let img = image::open(path.path()).unwrap();

		// copy with offset
		final_image.copy_from(&img, 0, top);
	}

	// create a directory for final results
	std::fs::create_dir("output").expect("Couldn't create write directory.");

	// save the image
	let ref mut fout = std::path::Path::new("output/knob.png");
	final_image.save(&fout).unwrap();

	// save a text file (needed by programs like kontakt)
	let data = format!("Has Alpha Channel: yes\nNumber of Animations: {}\nHorizontal Animation: no\nVertical Resizable: no\nHorizontal Resizable: no\nFixed Top: 0\nFixed Bottom: 0\nFixed Left: 0\nFixed Right: 0\n", paths.len());
	let mut metafile = std::fs::File::create("output/knob.txt").expect("Couldn't write image.");
	metafile.write_all(data.as_bytes()).expect("Couldn't write meta file.");

	// out
	println!("Finished.")

}
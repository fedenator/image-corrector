fn sacle_image(src_path: &std::path::PathBuf, dst_dir: &std::path::PathBuf)
{
	println!("Abriendo imagen {} ...", src_path.display());
	let src_img = image::open(src_path).unwrap();

	println!("Escalando...");
	let dst_image = src_img.resize(300, 400, image::imageops::FilterType::Lanczos3);
	
	let mut dst_path = dst_dir.clone();
	dst_path.push(std::path::PathBuf::from(src_path.file_name().unwrap().to_str().unwrap()));

	let mut dst_file = std::fs::OpenOptions::new()
		.create(true)
		.write(true)
		.open(&dst_path)
		.unwrap();
	
	println!("Guardando en {} ...", dst_path.display());
	dst_image.write_to(&mut dst_file, image::ImageFormat::Jpeg).unwrap();
}

fn main()
{
	println!("Iniciando corrector de imagenes...");
	let src_dir = std::path::PathBuf::from("/home/fpalacios/Documents/Drogueria la costa/fotos/originales/");
	let dst_dir = std::path::PathBuf::from("/home/fpalacios/Documents/Drogueria la costa/fotos/corregidas/");

	let dirs: Vec<std::fs::DirEntry> = std::fs::read_dir(src_dir).unwrap().map(|r| r.unwrap()).collect();

	for (index, src_file) in dirs.iter().enumerate()
	{
		println!("{}/{}", index, dirs.len());
		sacle_image(&src_file.path(), &dst_dir);
	}

	
	println!("Pronto el pollo");
}

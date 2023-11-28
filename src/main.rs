use image::{imageops::FilterType, DynamicImage, ImageFormat};

fn main() {
    let template: DynamicImage = image::open("./posters_template.png").unwrap();
    let p: Vec<DynamicImage> = std::fs::read_dir("./input")
        .unwrap()
        .flat_map(Result::ok)
        .map(|f| f.path())
        .map(image::open)
        .flat_map(Result::ok)
        .collect();

    std::fs::create_dir_all("./output/posters").unwrap();
    std::fs::create_dir_all("./output/tips").unwrap();

    for i in 0..p.len() {
        generate_atlas(
            &template,
            &[
                g(&p, i + 0),
                g(&p, i + 1),
                g(&p, i + 2),
                g(&p, i + 3),
                g(&p, i + 4),
            ],
        )
        .save_with_format(format!("./output/posters/{i}.png"), ImageFormat::Png)
        .unwrap();
    }
}

fn g<'a>(input: &'a Vec<DynamicImage>, index: usize) -> &'a DynamicImage {
    input.get(index % input.len()).unwrap()
}

const OFFSETS: &[&[u32; 4]; 5] = &[
    &[0, 0, 341, 559],
    &[346, 0, 284, 559],
    &[641, 58, 274, 243],
    &[184, 620, 411, 364],
    &[632, 320, 372, 672],
];

fn generate_atlas(template: &DynamicImage, posters: &[&DynamicImage; 5]) -> DynamicImage {
    let mut base = template.clone();
    for (i, o) in OFFSETS.iter().enumerate() {
        let p = posters[i].resize(o[2], o[3], FilterType::Lanczos3);
        image::imageops::overlay(&mut base, &p, (o[0] + p.width() - o[2]) as i64, o[1] as i64);
    }
    base
}

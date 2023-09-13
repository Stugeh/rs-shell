pub mod font_loader{
    use std::collections::HashMap;
    use fontdue::Metrics;


    pub struct Glyph{
        pub metrics: Metrics,
        pub glyph_bytes: Vec<u8>
    }


    pub fn get_glyph_table() -> HashMap<char, Glyph>{
        let font_bytes = include_bytes!("../JetBrainsMonoNL-SemiBold.ttf") as &[u8];

        let font = fontdue::Font::from_bytes(font_bytes, fontdue::FontSettings::default()).expect("Failed to initialize font");
        let mut rasterized_glyphs: HashMap<char, Glyph>= HashMap::new();

        let chars=font.chars();

        for (ch,_) in chars{ 
            let (mut metrics, glyph_bytes) = font.rasterize(*ch, 128.0);
            if *ch ==' ' {metrics.width = (128.0 / 2.0) as usize};
            rasterized_glyphs.insert(*ch, create_new_glyph((metrics,glyph_bytes))); 
        };

        rasterized_glyphs
    }

    pub fn get_max_glyph_height(glyph_map:&HashMap<char,Glyph>)-> usize{

        let tallest_glyph = glyph_map.values().max_by_key(|x| x.metrics.height).expect("getting tallest_glyph failed");
        tallest_glyph.metrics.height
    }

    fn create_new_glyph((metrics, glyph_bytes):(Metrics, Vec<u8>))-> Glyph{
        Glyph{
            metrics, 
            glyph_bytes
        }

    }
}

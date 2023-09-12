pub mod font_loader{
    use std::collections::HashMap;
    use fontdue::Metrics;


    pub struct Glyph{
        metrics: Metrics,
        glyph_bytes: Vec<u8>
    }


    pub fn get_glyph_table() -> HashMap<char, Glyph>{
        let font_bytes = include_bytes!("../JetBrainsMonoNL-SemiBold.ttf") as &[u8];

        let font = fontdue::Font::from_bytes(font_bytes, fontdue::FontSettings::default()).expect("Failed to initialize font");
        let mut rasterized_glyphs: HashMap<char, Glyph>= HashMap::new();

        let chars=font.chars();

        for (ch,_) in chars{ rasterized_glyphs.insert(*ch, create_new_glyph(font.rasterize(*ch, 17.0))); };

        rasterized_glyphs
    }



    fn create_new_glyph((metrics, glyph_bytes):(Metrics, Vec<u8>))-> Glyph{
        Glyph{
            metrics, 
            glyph_bytes
        }

    }
    // fn temp (){
    // let font = include_bytes!("../JetBrainsMonoNL-SemiBold.ttf") as &[u8];
    // 

    // let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
    // let (metrics, glyph) = font.rasterize('a', 17.0);

    // let mut row_offset = 0;
    // for (idx, byte) in glyph.iter().enumerate(){
    //     if metrics.width % idx == 0 {row_offset+= 1;}

    //     if *byte > 0 {
    //         buffer[idx+row_offset*window.inner_size().width]
    //     }}
    // }
}

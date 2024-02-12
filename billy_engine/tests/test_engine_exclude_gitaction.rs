#[cfg(test)]
mod teste_billy_engine_gitaction {
    extern crate billy_engine;
    use billy_engine::engine::ScreenData;

    #[test]
    fn test_screen_data() {
        const OFFSET: u8 = 1;
        let mut sd = ScreenData::new();
        let h: u16 = sd.get_height() - OFFSET as u16 ;
        sd.set_offset(OFFSET);
        assert_eq!(OFFSET, sd.get_offset());
        assert_eq!(h, sd.get_height());
    }
}


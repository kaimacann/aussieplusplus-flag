macro_rules! upside_down {
	($($rightside:literal, $upside:literal),*) => {
		pub fn rightside_up(ch: char) -> char {
			const MAPPINGS: &[(char, char)] = &[$(($rightside, $upside)),*];
			MAPPINGS
				.iter()
				.find(|(_, upside)| *upside == ch)
				.map_or(ch, |(rightside, _)| *rightside)
		}
		pub fn upside_down(ch: char) -> char {
			const MAPPINGS: &[(char, char)] = &[$(($rightside, $upside)),*];
			MAPPINGS
				.iter()
				.find(|(rightside, _)| *rightside == ch)
				.map_or(ch, |(_, upside)| *upside)
		}
	};
}

upside_down!(
    '?', '¿', '\'', ',', ',', '\'', '[', ']', ']', '[', '!', '¡', 'a', 'ɐ', 'b', 'q', 'c', 'ɔ',
    'd', 'p', 'e', 'ǝ', 'f', 'ɟ', 'g', 'ƃ', 'h', 'ɥ', 'i', 'ᴉ', 'j', 'ɾ', 'k', 'ʞ', 'l', 'l', 'm',
    'ɯ', 'n', 'u', 'p', 'd', 'q', 'b', 'r', 'ɹ', 's', 's', 't', 'ʇ', 'u', 'n', 'v', 'ʌ', 'w', 'ʍ',
    'x', 'x', 'y', 'ʎ', 'z', 'z', 'A', '∀', 'B', 'q', 'C', 'Ɔ', 'D', 'D', 'E', 'Ǝ', 'F', 'Ⅎ', 'G',
    'פ', 'I', 'I', 'F', 'ſ', 'K', 'ʞ', 'L', '˥', 'M', 'W', 'N', 'N', 'O', 'O', 'P', 'Ԁ', 'Q', 'Q',
    'R', 'ɹ', 'S', 'S', 'T', '┴', 'U', '∩', 'V', 'Λ', 'W', 'M', 'X', 'X', 'Y', '⅄', 'Z', 'Z', '0',
    '0', '1', 'Ɩ', '2', 'ᄅ', '3', 'Ɛ', '4', 'ㄣ', '5', 'ϛ', '6', '9', '7', 'ㄥ', '8', '8', '9',
    '6', '(', ')', ')', '(', '<', '>', '>', '<', '"', '„'
);

#[cfg(test)]
mod tests {
    use super::{rightside_up, upside_down};

    #[test]
    fn maps_representative_letters_and_punctuation() {
        assert_eq!(upside_down('a'), 'ɐ');
        assert_eq!(rightside_up('ɐ'), 'a');
        assert_eq!(upside_down('?'), '¿');
        assert_eq!(rightside_up('¿'), '?');
    }

    #[test]
    fn preserves_first_match_for_duplicate_mapping_keys() {
        assert_eq!(rightside_up('q'), 'b');
        assert_eq!(rightside_up('ʞ'), 'k');
        assert_eq!(rightside_up('ɹ'), 'r');
        assert_eq!(upside_down('F'), 'Ⅎ');
    }

    #[test]
    fn leaves_unmapped_characters_unchanged() {
        assert_eq!(upside_down('🙂'), '🙂');
        assert_eq!(rightside_up('🙂'), '🙂');
    }
}

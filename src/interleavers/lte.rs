use super::qpp::Qpp;

/// LTE QPP Interleaver.
pub struct LteQpp;

impl LteQpp {
    const F_K5_K64_STEP1: [(u8, u8); 60] = [
        (3, 10),
        (7, 12),
        (19, 42),
        (7, 16),
        (7, 18),
        (11, 20),
        (5, 22),
        (11, 24),
        (7, 26),
        (41, 84),
        (103, 90),
        (15, 32),
        (9, 34),
        (17, 108),
        (9, 38),
        (21, 120),
        (101, 84),
        (21, 44),
        (57, 46),
        (23, 48),
        (13, 50),
        (27, 52),
        (11, 36),
        (27, 56),
        (85, 58),
        (29, 60),
        (33, 62),
        (15, 32),
        (17, 198),
        (33, 68),
        (103, 210),
        (19, 36),
        (19, 74),
        (37, 76),
        (19, 78),
        (21, 120),
        (21, 82),
        (115, 84),
        (193, 86),
        (21, 44),
        (133, 90),
        (81, 46),
        (45, 94),
        (23, 48),
        (243, 98),
        (151, 40),
        (155, 102),
        (25, 52),
        (51, 106),
        (47, 72),
        (91, 110),
        (29, 168),
        (29, 114),
        (247, 58),
        (29, 118),
        (89, 180),
        (91, 122),
        (157, 62),
        (55, 84),
        (31, 64),
    ];

    const F_K66_K128_STEP2: [(u8, u16); 32] = [
        (17, 66),
        (35, 68),
        (227, 420),
        (65, 96),
        (19, 74),
        (37, 76),
        (41, 234),
        (39, 80),
        (185, 82),
        (43, 252),
        (21, 86),
        (155, 44),
        (79, 120),
        (139, 92),
        (23, 94),
        (217, 48),
        (25, 98),
        (17, 80),
        (127, 102),
        (25, 52),
        (239, 106),
        (17, 48),
        (137, 110),
        (215, 112),
        (29, 114),
        (15, 58),
        (147, 118),
        (29, 60),
        (59, 122),
        (65, 124),
        (55, 84),
        (31, 64),
    ];

    const F_K132_K256_STEP4: [(u8, u16); 32] = [
        (17, 66),
        (171, 204),
        (67, 140),
        (35, 72),
        (19, 74),
        (39, 76),
        (19, 78),
        (199, 240),
        (21, 82),
        (211, 252),
        (21, 86),
        (43, 88),
        (149, 60),
        (45, 92),
        (49, 846),
        (71, 48),
        (13, 28),
        (17, 80),
        (25, 102),
        (183, 104),
        (55, 954),
        (127, 96),
        (27, 110),
        (29, 112),
        (29, 114),
        (57, 116),
        (45, 354),
        (31, 120),
        (59, 610),
        (185, 124),
        (113, 420),
        (31, 64),
    ];

    const F_K264_K768_STEP8: [(u16, u16); 64] = [
        (17, 66),
        (171, 136),
        (209, 420),
        (253, 216),
        (367, 444),
        (265, 456),
        (181, 468),
        (39, 80),
        (27, 164),
        (127, 504),
        (143, 172),
        (43, 88),
        (29, 300),
        (45, 92),
        (157, 188),
        (47, 96),
        (13, 28),
        (111, 240),
        (443, 204),
        (51, 104),
        (51, 212),
        (451, 192),
        (257, 220),
        (57, 336),
        (313, 228),
        (271, 232),
        (179, 236),
        (331, 120),
        (363, 244),
        (375, 248),
        (127, 168),
        (31, 64),
        (33, 130),
        (43, 264),
        (33, 134),
        (477, 408),
        (35, 138),
        (233, 280),
        (357, 142),
        (337, 480),
        (37, 146),
        (71, 444),
        (71, 120),
        (37, 152),
        (39, 462),
        (127, 234),
        (39, 158),
        (39, 80),
        (31, 96),
        (113, 902),
        (41, 166),
        (251, 336),
        (43, 170),
        (21, 86),
        (43, 174),
        (45, 176),
        (45, 178),
        (161, 120),
        (89, 182),
        (323, 184),
        (47, 186),
        (23, 94),
        (47, 190),
        (263, 480),
    ];

    /// Get an interleaver for the block length `k` (in bits).
    pub fn get(k: usize) -> Option<Qpp> {
        Self::get_params(k).map(|(f1, f2)| Qpp::new(k, f1, f2))
    }

    fn get_params(k: usize) -> Option<(usize, usize)> {
        if (5 * 8..=64 * 8).contains(&k) {
            let index = (k - 5 * 8) / 8;
            if 5 * 8 + index * 8 == k {
                let (f1, f2) = Self::F_K5_K64_STEP1[index];
                Some((f1 as usize, f2 as usize))
            } else {
                None
            }
        } else if (66 * 8..=128 * 8).contains(&k) {
            let index = (k - 66 * 8) / 16;
            if 66 * 8 + index * 16 == k {
                let (f1, f2) = Self::F_K66_K128_STEP2[index];
                Some((f1 as usize, f2 as usize))
            } else {
                None
            }
        } else if (132 * 8..=256 * 8).contains(&k) {
            let index = (k - 132 * 8) / 32;
            if 132 * 8 + index * 32 == k {
                let (f1, f2) = Self::F_K132_K256_STEP4[index];
                Some((f1 as usize, f2 as usize))
            } else {
                None
            }
        } else if (264 * 8..=768 * 8).contains(&k) {
            let index = (k - 264 * 8) / 64;
            if 264 * 8 + index * 64 == k {
                let (f1, f2) = Self::F_K264_K768_STEP8[index];
                Some((f1 as usize, f2 as usize))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn get_params() {
        assert_eq!(None, LteQpp::get_params(4 * 8));
        assert_eq!(Some((3, 10)), LteQpp::get_params(5 * 8));
        assert_eq!(Some((31, 64)), LteQpp::get_params(64 * 8));
        assert_eq!(None, LteQpp::get_params(65 * 8));
        assert_eq!(Some((17, 66)), LteQpp::get_params(66 * 8));
        assert_eq!(None, LteQpp::get_params(67 * 8));
        assert_eq!(Some((31, 64)), LteQpp::get_params(128 * 8));
        assert_eq!(None, LteQpp::get_params(129 * 8));
        assert_eq!(None, LteQpp::get_params(131 * 8));
        assert_eq!(Some((17, 66)), LteQpp::get_params(132 * 8));
        assert_eq!(None, LteQpp::get_params(133 * 8));
        assert_eq!(Some((31, 64)), LteQpp::get_params(256 * 8));
        assert_eq!(None, LteQpp::get_params(257 * 8));
        assert_eq!(None, LteQpp::get_params(263 * 8));
        assert_eq!(Some((17, 66)), LteQpp::get_params(264 * 8));
        assert_eq!(Some((263, 480)), LteQpp::get_params(768 * 8));
        assert_eq!(None, LteQpp::get_params(769 * 8));
    }
}

pub fn parse_fast_whitespeace_separated<const L: usize, const N: usize>(
    input: &[u8],
) -> [[i32; L]; N] {
    let mut result = [[0; L]; N];

    let mut n = N - 1;
    let mut l = L - 1;

    let mut next_base = 1;
    let mut decimal = 0;

    for byte in input.into_iter().rev().skip(1) {
        match byte {
            // whitespace
            0x20 => {
                result[n][l] = decimal;
                decimal = 0;
                next_base = 1;
                l -= 1;
            }
            // line-feed
            0x0A => {
                result[n][l] = decimal;
                decimal = 0;
                next_base = 1;
                n -= 1;
                l = L - 1;
            }
            0x30 => {
                next_base *= 10;
            }
            0x31 => {
                decimal += next_base * 1;
                next_base *= 10;
            }
            0x32 => {
                decimal += next_base * 2;
                next_base *= 10;
            }
            0x33 => {
                decimal += next_base * 3;
                next_base *= 10;
            }
            0x34 => {
                decimal += next_base * 4;
                next_base *= 10;
            }
            0x35 => {
                decimal += next_base * 5;
                next_base *= 10;
            }
            0x36 => {
                decimal += next_base * 6;
                next_base *= 10;
            }
            0x37 => {
                decimal += next_base * 7;
                next_base *= 10;
            }
            0x38 => {
                decimal += next_base * 8;
                next_base *= 10;
            }
            0x39 => {
                decimal += next_base * 9;
                next_base *= 10;
            }
            _ => unreachable!(),
        }
    }

    result[n][l] = decimal;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    17 26 34 42 51
    1 2 7 8
    9 7 6 2 1
    1 3 2 4 5
    8 6
    1 3 6 7 9
    */

    const TEST_DATA: &[u8] = &[
        0x31, 0x37, 0x20, 0x32, 0x36, 0x20, 0x33, 0x34, 0x20, 0x34, 0x32, 0x20, 0x35, 0x31, 0x0A,
        0x31, 0x20, 0x32, 0x20, 0x37, 0x20, 0x38, 0x0A, 0x39, 0x20, 0x37, 0x20, 0x36, 0x20, 0x32,
        0x20, 0x31, 0x0A, 0x31, 0x20, 0x33, 0x20, 0x32, 0x20, 0x34, 0x20, 0x35, 0x0A, 0x38, 0x20,
        0x36, 0x0A, 0x31, 0x20, 0x33, 0x20, 0x36, 0x20, 0x37, 0x20, 0x39, 0x0A,
    ];

    #[test]
    fn test_fast_parse() {
        let data = parse_fast_whitespeace_separated::<5, 6>(TEST_DATA);
        assert_eq!(
            data,
            [
                [17, 26, 34, 42, 51],
                [0, 1, 2, 7, 8],
                [9, 7, 6, 2, 1],
                [1, 3, 2, 4, 5],
                [0, 0, 0, 8, 6],
                [1, 3, 6, 7, 9],
            ]
        );
    }
}

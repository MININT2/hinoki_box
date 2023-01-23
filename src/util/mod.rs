pub(crate) mod instruction;

use twox_hash::XxHash32;
use std::{
    collections::HashMap,
    char,
    hash::BuildHasherDefault,
    cmp::min
};

//stringLib
//string matching
//not optimized, damlev is exhaustive

//helper glue functions
fn min4(a: usize, b: usize, c: usize, d: usize) -> usize {
    //yoink out the smallest
    return min(min(min(a, b), c), d);
}

//string distance measuring algos
pub fn damlev_distance(a: &str, b: &str) -> usize {
    let len_a = a.chars().count();
    let len_b = b.chars().count();
    let max_distance = len_a + len_b;

    //init the matrix
    let mut matty: Vec<Vec<usize>> = vec![vec![0; len_b + 2]; len_a + 2];
    //store max distance in the matrix
    matty[0][0] = max_distance;
    //interleave
    for i in 0..(len_a + 1) {
        matty[i + 1][0] = max_distance;
        matty[i + 1][1] = i;
    }
    for i in 0..(len_b + 1) {
        matty[0][i + 1] = max_distance;
        matty[1][i + 1] = i;
    }
    //actually do the shit, matching and calculating distances for various string transformations
    let mut char_map: HashMap<char, usize, BuildHasherDefault<XxHash32>> = Default::default();
    for (i, a_char) in a.chars().enumerate() {
        let mut db = 0;
        let i = i + 1;

        for (j, b_char) in b.chars().enumerate() {
            let j = j + 1;
            let last = *char_map.get(&b_char).unwrap_or(&0);

            let cost = if a_char == b_char { 0 } else { 1 };
            matty[i + 1][j + 1] = min4(
                matty[i + 1][j] + 1, //deletes
                matty[i][j + 1] + 1, //inserts
                matty[i][j] + cost, //substitutes
                matty[last][db] + (i - last - 1) + 1 + (j - db - 1) //transposes
            );

            //a_char == b_char but more efficient
            if cost == 0 {
                db = j;
            }
        }
        char_map.insert(a_char, i);
    }
    let ret = matty[len_a + 1][len_b + 1];
    println!("damlev distance for: {}", a.to_owned() + " " + b + " | # = " + &*ret.to_string());
    return ret;
}
pub fn sift3_distance(a: &str, b: &str, max_offset: usize) -> f32 {
    let len_a = a.chars().count();
    let len_b = b.chars().count();
    let av: Vec<char> = a.chars().collect();
    let bv: Vec<char> = b.chars().collect();
    let mut c = 0;
    let mut offset1 = 0;
    let mut offset2 = 0;
    let mut lcs = 0;
    // handle empty strings
    if len_a == 0 {
        if len_b == 0 {
            return 0.0;
        } else {
            return len_b as f32;
        }
    }
    if len_b == 0 {
        return len_a as f32;
    }
    //everything else
    while (c + offset1 < len_a) && (c + offset2 < len_b) {
        if av[c + offset1] == bv[c + offset2] {
            lcs += 1;
        } else {
            offset1 = 0;
            offset2 = 0;
            for i in 0..max_offset {
                if (c + i < len_a) && av[c + i] == bv[c] {
                    offset1 = i;
                    break;
                }

                if (c + i < len_b) && (bv[c] == bv[c + i]) {
                    offset2 = i;
                    break;
                }
            }
        }
        c += 1;
    }
    let ret = ((len_a + len_b) as f32) / 2.0 - (lcs as f32);
    println!("sift3 magnitude for: {}", a.to_owned() + " " + b + " | % = " + &*ret.to_string());
    return ret;
}

//string matching algos
//fn bitap_match(t: &str, p: &str) {
    //use damlev for this
//}

#[cfg(test)]
mod tests {
    use super::{
        damlev_distance,
        sift3_distance,
    };

    #[test]
    fn basic() {
        assert_eq!(1, damlev_distance("hannah", "hannha"));
        assert_eq!(2, damlev_distance("FOO", "BOR"));
        assert_eq!(1, damlev_distance("BAR", "BOR"));
        assert_eq!(1, damlev_distance("hansi", "hasni"));
        assert_eq!(2, damlev_distance("zzaabbio", "zzababoi"));
        assert_eq!(1, damlev_distance("zzaabb", "zzabab"));
        assert_eq!(3, damlev_distance("abcdef", "badcfe"));
        assert_eq!(1, damlev_distance("klmb", "klm"));
        assert_eq!(1, damlev_distance("klm", "klmb"));
        assert_eq!(0.5, sift3_distance("hannah", "hanna", 5));
        assert_eq!(2.5, sift3_distance("kitten", "sitting", 5));
        assert_eq!(2.0, sift3_distance("book", "back", 5));
        assert_eq!(4.5, sift3_distance("table", "dinner", 5));
        assert_eq!(2.0, sift3_distance("person", "pardon", 5));
        assert_eq!(0.5, sift3_distance("person", "persons", 5));
    }

    #[test]
    fn empty() {
        assert_eq!(0, damlev_distance("", ""));
        assert_eq!(3, damlev_distance("", "foo"));
        assert_eq!(3, damlev_distance("bar", ""));
        assert_eq!(4.0, sift3_distance("book", "", 5));
        assert_eq!(4.0, sift3_distance("", "book", 5));
        assert_eq!(0.0, sift3_distance("", "", 5));
    }

    #[test]
    fn cases() {
        assert_eq!(1, damlev_distance("Hello", "hello"));
        assert_eq!(1, damlev_distance("World", "world"));
        assert_eq!(1.0, sift3_distance("Hello", "hello", 5));
        assert_eq!(1.0, sift3_distance("World", "world", 5));
    }

    #[test]
    fn same() {
        assert_eq!(0, damlev_distance("pomme de terre", "pomme de terre"));
        assert_eq!(0.0, sift3_distance("kitten", "kitten", 5));
        assert_eq!(0.0, sift3_distance("a", "a", 5));
        assert_eq!(0.0, sift3_distance("ち", "ち", 5));
    }

    #[test]
    fn reversed() {
        assert_eq!(5, damlev_distance("damerau", "iiqjau"));
        assert_eq!(5, damlev_distance("iiqjau", "damerau"));
    }

    #[test]
    fn lorem() {
        assert_eq!(80, damlev_distance(
            "Lorem ipsum dolor sit amet, autem mucius eirmod ea per. Nec adhuc laudem id, vix dolor interesset ea.",
            "Id mundi ponderum constituam nam. Nam in legendos definiebas. Pri commune senserit omittantur cu.")
        );
        assert_eq!(93.0, sift3_distance(
            "Lorem ipsum dolor sit amet, autem mucius eirmod ea per. Nec adhuc laudem id, vix dolor interesset ea.",
            "Id mundi ponderum constituam nam. Nam in legendos definiebas. Pri commune senserit omittantur cu.", 5)
        );
    }

    #[test]
    fn unicode() {
        assert_eq!(6, damlev_distance("さようなら", "༆˥ʘöpa"));
        assert_eq!(3, damlev_distance("abc", "öঙ香"));
        assert_eq!(3, damlev_distance("", "öঙ香"));
        assert_eq!(1, damlev_distance("よさ", "äさ"));
        assert_eq!(1.5, sift3_distance("Späße", "Spaß", 5));
        assert_eq!(5.0, sift3_distance("さようなら", "こんにちは", 5));
        assert_eq!(1.0, sift3_distance("さようなら", "さようなう", 5));
        assert_eq!(2.0, sift3_distance("こんにちは", "こんにちは abc", 5));
        assert_eq!(1.0, sift3_distance("༆༃ʘ", "༆˥ʘ", 5));
    }
}
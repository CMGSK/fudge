use std::collections::HashMap;

use crate::components::config::Configuration;

pub fn find_by_custom_levenshtein(cfg: Configuration, command: String) -> String {
    let (_simil, guess) = cfg
        .haystack
        .iter()
        .map(|c| {
            let len1 = c.len();
            let len2 = command.len();
            let lenm = if (len1 as usize).abs_diff(len2 as usize) <= 1 {
                2 as usize
            } else {
                1 as usize
            };
            let mut dist = vec![vec![0; len2 + 1]; len1 + 1];

            for i in 0..=len1 {
                for j in 0..=len2 {
                    if i == 0 {
                        dist[i][j] = j * lenm;
                    } else if j == 0 {
                        dist[i][j] = i * lenm;
                    } else {
                        let p = if command.chars().nth(i - 1) == c.chars().nth(j - 1) {
                            0
                        } else {
                            let base = 1;

                            // Added weight to first and last characters
                            let fol_weight = if i == 1 || j == 1 || i == len1 || j == len2 {
                                2
                            } else {
                                1
                            };

                            // Added weight to order significance to benefit character swapping errors
                            let ord_weight = (i as usize).abs_diff(j as usize) as usize;

                            (base * fol_weight + ord_weight) * lenm
                        };

                        dist[i][j] = std::cmp::min(
                            std::cmp::min(dist[i - 1][j] + lenm, dist[i][j - 1] + lenm),
                            dist[i - 1][j - 1] + p,
                        );
                    }
                }
            }

            (dist[len1][len2], c.clone())
        })
        .min_by_key(|(w, s)| (w.clone(), s.clone()))
        .unwrap();
    guess
}

pub fn edit_distance(source: &str, text: &str) -> usize {
    let a = source.chars().collect::<Vec<_>>();
    let b = text.chars().collect::<Vec<_>>();
    let m = a.len();
    let n = b.len();

    let mut prev = 0;
    let mut curr = (0..=n + 1).collect::<Vec<_>>();
    for (i, &ca) in a.iter().enumerate() {
        prev = curr[0];
        curr[0] = i + 1;
        for (j, &cb) in b.iter().enumerate() {
            let temp = curr[j + 1];
            if ca == cb {
                curr[i + 1] = prev;
            } else {
                curr[j + 1] = 1 + prev.min(curr[j]).min(curr[j + 1]);
            }

            prev = temp;
        }
    }

    curr[n]
}

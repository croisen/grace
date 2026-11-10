pub static EDIT_THRESHOLD: usize = 3; // up to two mistakes

pub fn edit_distance_sen(source: &str, text: &str) -> usize {
    let a = source.chars().collect::<Vec<_>>();
    let b = text.chars().collect::<Vec<_>>();
    let n = b.len();

    let mut prev;
    let mut curr = (0..=n + 1).collect::<Vec<_>>();
    for (i, &ca) in a.iter().enumerate() {
        prev = curr[0];
        curr[0] = i + 1;
        for (j, &cb) in b.iter().enumerate() {
            let temp = curr[j + 1];
            if ca == cb {
                curr[j + 1] = prev;
            } else {
                curr[j + 1] = 1 + prev.min(curr[j]).min(curr[j + 1]);
            }

            prev = temp;
        }
    }

    curr[n]
}

pub fn edit_distance_ins(source: &str, text: &str) -> usize {
    let a = source.chars().collect::<Vec<_>>();
    let b = text.chars().collect::<Vec<_>>();
    let n = b.len();

    let mut prev;
    let mut curr = (0..=n + 1).collect::<Vec<_>>();
    for (i, &ca) in a.iter().enumerate() {
        prev = curr[0];
        curr[0] = i + 1;
        for (j, &cb) in b.iter().enumerate() {
            let temp = curr[j + 1];
            if ca.eq_ignore_ascii_case(&cb) {
                curr[j + 1] = prev;
            } else {
                curr[j + 1] = 1 + prev.min(curr[j]).min(curr[j + 1]);
            }

            prev = temp;
        }
    }

    curr[n]
}

pub fn close_enough(source: &str, distance: usize) -> bool {
    let threshold = EDIT_THRESHOLD.min(source.len());
    distance == 0 || distance < threshold
}

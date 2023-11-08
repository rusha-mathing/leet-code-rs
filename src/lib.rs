pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let mut rows: Vec<String> = vec![String::new(); num_rows];
        (0..num_rows)
            .chain((1..num_rows - 1).rev())
            .cycle()
            .zip(s.chars())
            .for_each(|(i, ch)| {
                rows[i].push(ch);
            });
        rows.concat().to_owned()
    }
}

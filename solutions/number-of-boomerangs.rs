impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut ans: i32 = 0;
        let n: usize = points.len();
        for i in 0..n {
            use std::collections::HashMap;
            let mut map: HashMap<i32, i32> = HashMap::new();
            for j in 0..n {
                let distance = (points[i][0] - points[j][0]).pow(2) + (points[i][1] - points[j][1]).pow(2);
                *map.entry(distance).or_insert(0) += 1;
            }
            for (_, val) in map.iter() {
                ans += val * (val - 1);
            }
        }
        ans
    }
}
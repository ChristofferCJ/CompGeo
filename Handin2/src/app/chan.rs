use crate::Point;

fn chan(points: Vec<Point>) -> Option<Vec<Point>> {
    let n = points.len() as f32;
    for i in 0..=n.log2().log2().ceil() as usize {
        let guess = uh_with_size(&points, i.pow(2).pow(2));
        match guess {
            Some(res) => return Some(res),
            None => ()
        }
    }
    return None;
}

fn uh_with_size(points: &Vec<Point>, size: usize) -> Option<Vec<Point>> {
    todo!()
}
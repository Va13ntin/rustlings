// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

///il faut ajouter un pattern _ à la branche None du match pour traiter le cas où y est None. 
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        None => panic!("no match!"), // Ajouter ce pattern pour traiter le cas None
    }
    y; // Fix without deleting this line.
}


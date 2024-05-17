// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_some() {
       /// Puisque nous voulons simplement vérifier si c'est `None`, nous pouvons simplement utiliser `if let`.
    }

    let my_arr = &[
        -1, -2, -3, /// Ajout d'une virgule pour séparer correctement les éléments
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = Vec::new(); /// Ajout d'une annotation de type
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}

use std::time::Instant;

fn print_elements(elements: &Vec<String>) {
    // for element in elements {
    //     println!("{}", element)
    // }

    elements.iter()
        .enumerate()
        .map(|(index, el)| {
            if elements.len() - 1 == index {
                format!("{} {}", el, elements[0])
            } else {
                format!("{} {}", el, elements[index + 1])
            }
        })
        .for_each(|el| println!("{}", el))
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}

fn move_elements(move_a: Vec<String>, move_b: &mut Vec<String>) {
    move_a.into_iter().for_each(|el| move_b.push(el))
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(
            |el| el.chars().map(|c| c.to_string()).collect()
        )
        .collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(fallback.to_string(), |el| el.to_string())
}

fn main() {
    let start = Instant::now();
    let colors = vec![
        "red".to_string(),
        "green".to_string(),
        "blue".to_string(),
    ];

    // shorten_strings(&mut colors);
    // println!("{:#?}", colors);

    // let uppercased = to_uppercase(&colors);
    // println!("{:#?}", uppercased);

    // let mut destination = vec![];
    // move_elements(colors, &mut destination);
    // println!("{:#?}", destination);

    // let exploded = explode(&colors);
    // println!("{:#?}", exploded);

    let found_color = find_color_or(&colors, "re", "blue");
    println!("{:#?}", found_color);

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

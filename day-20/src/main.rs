fn main() {
    let input = std::io::stdin();

    let mut encrypted = input
        .lines()
        .filter_map(|v| v.map(|v| v.parse::<i32>().ok()).ok().flatten())
        .collect::<Vec<_>>();

    let mut indices = Vec::with_capacity(encrypted.len());
    for i in 0..encrypted.len() {
        indices.push(i);
    }

    let mut current_index = 0;

    while current_index < encrypted.len() {
        let (move_index, _) = indices
            .iter()
            .enumerate()
            .find(|v| *v.1 == current_index)
            .unwrap();
        let value = encrypted[move_index];
        move_element(&mut encrypted, move_index, value);
        move_element(&mut indices, move_index, value);

        current_index += 1;
    }

    let (zero_index, _) = encrypted.iter().enumerate().find(|v| *v.1 == 0).unwrap();

    let mut acc = 0;

    for i in 1..=3 {
        let mut new_index = zero_index as i32 + i * 1000;

        while new_index < 0 {
            new_index = encrypted.len() as i32 + new_index;
        }

        while new_index > encrypted.len() as i32 {
            new_index = new_index - encrypted.len() as i32;
        }
        acc += dbg!(encrypted[new_index as usize]);
    }

    dbg!(acc);
}

fn move_element<T>(list: &mut Vec<T>, index: usize, amount: i32) {
    let removed = list.remove(index);

    let mut new_index = index as i32 + amount;

    while new_index < 0 {
        new_index = list.len() as i32 + new_index;
    }

    while new_index > list.len() as i32 {
        new_index = new_index - list.len() as i32;
    }
    list.insert(new_index as usize, removed);
}

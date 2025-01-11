use std::vec;

// Get all cords that are visible from the top left corner of the map.
pub fn search_cords(search_size: usize) -> (Vec<Vec<Vec<(i16, i16, f32)>>>, Vec<(i16, i16, f32)>) {
    let mut cords = Vec::new();
    let mut search_area = Vec::new();
    for _ in 0..search_size {
        let mut line = Vec::new();
        line.resize(search_size as usize, false);
        search_area.push(line);
    }

    for round in 1..search_size {
        let mut layer = Vec::new();

        let x = round;
        for y in 1..round + 1 {
            if search_area[y][x] {
                continue;
            }

            search_area[y][x] = true;
            layer.push((
                x as i16,
                y as i16,
                (y as f32 / x as f32).atan().to_degrees(),
            ));

            let mut y_offset = y * 2;
            let mut x_offset = x * 2;
            while y_offset < search_size && x_offset < search_size {
                search_area[y_offset][x_offset] = true;
                y_offset += y;
                x_offset += x;
            }
        }

        let y = round;
        for x in 1..round {
            if search_area[y][x] {
                continue;
            }

            search_area[y][x] = true;
            layer.push((
                x as i16,
                y as i16,
                (y as f32 / x as f32).atan().to_degrees(),
            ));

            let mut y_offset = y * 2;
            let mut x_offset = x * 2;
            while y_offset < search_size && x_offset < search_size {
                search_area[y_offset][x_offset] = true;
                y_offset += y;
                x_offset += x;
            }
        }

        cords.push(layer);
    }

    // Generate the cords for part 1 of the puzzle
    let cords_len = cords.len();
    let mut cords_looking_layers = vec![cords.clone(), cords.clone(), cords.clone(), cords.clone()];

    for j in 0..cords_len {
        for k in 0..cords_looking_layers[0][j].len() {
            let (x, y, angle) = cords_looking_layers[0][j][k];

            cords_looking_layers[1][j][k] = (-x, y, angle);
            cords_looking_layers[2][j][k] = (x, -y, angle);
            cords_looking_layers[3][j][k] = (-x, -y, angle);
        }
    }

    cords_looking_layers.push(vec![vec![
        (0, 1, 0f32),
        (0, -1, 0f32),
        (1, 0, 0f32),
        (-1, 0, 0f32),
    ]]);

    // Generate the cords for part 2 of the puzzle
    let mut flat_cords: Vec<(i16, i16, f32)> = cords.iter().flatten().cloned().collect();
    flat_cords.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    let mut cords_in_circle = Vec::new();
    // Insert the top right cords
    cords_in_circle.push((0, -1, 0f32));
    for (x, y, angle) in flat_cords.iter() {
        cords_in_circle.push((*x, -*y, *angle));
    }
    // Insert the bottom right cords
    cords_in_circle.push((1, 0, 90f32));
    for (x, y, angle) in flat_cords.iter() {
        cords_in_circle.push((*y, *x, *angle + 90f32));
    }
    // Insert the bottom left cords
    cords_in_circle.push((0, 1, 180f32));
    for (x, y, angle) in flat_cords.iter() {
        cords_in_circle.push((-*x, *y, *angle + 180f32));
    }
    // Insert the top left cords
    cords_in_circle.push((-1, 0, 270f32));
    for (x, y, angle) in flat_cords.iter() {
        cords_in_circle.push((-*y, -*x, *angle + 270f32));
    }

    (cords_looking_layers, cords_in_circle)
}

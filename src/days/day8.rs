use ndarray::prelude::*;
use ndarray::{Array, Dim, OwnedRepr};
use std::fs;
use std::usize::MAX as USIZE_MAX;

const HEIGHT: i64 = 6;
const WIDTH: i64 = 25;

pub fn main() {
    let data = fs::read_to_string("./src/data/day8.txt").unwrap();
    let data = data.trim().to_string();
    let layers = get_layers(data);
    let out = find_least_zeros(&layers);
    println!("Part 1: {}", out);
    let image = find_image(&layers);
    for row in image.outer_iter() {
        let row: Vec<&str> = row
            .iter()
            .map(|x| if *x == 0 as i64 { "." } else { "#" })
            .collect();
        println!("{:?}", row.concat());
    }
}

fn find_image(
    layers: &ArrayBase<OwnedRepr<i64>, Dim<[usize; 3]>>,
) -> ArrayBase<OwnedRepr<i64>, Dim<[usize; 2]>> {
    let mut image = Array::<i64, _>::zeros((HEIGHT as usize, WIDTH as usize));
    image.fill(2);
    for layer in layers.outer_iter() {
        for (index, elt) in layer.indexed_iter() {
            if image[index] == 2 {
                if *elt != 2 {
                    image[index] = *elt;
                }
            }
        }
    }
    image
}

fn find_least_zeros(layers: &ArrayBase<OwnedRepr<i64>, Dim<[usize; 3]>>) -> usize {
    let mut min = USIZE_MAX;
    let mut mult = 0;
    for layer in layers.outer_iter() {
        let zeros: Vec<&i64> = layer.iter().filter(|x| **x == 0).collect();
        if zeros.len() < min {
            min = zeros.len();
            let ones = layer
                .iter()
                .filter(|x| **x == 1)
                .collect::<Vec<&i64>>()
                .len();
            let twos = layer
                .iter()
                .filter(|x| **x == 2)
                .collect::<Vec<&i64>>()
                .len();
            mult = ones * twos;
        }
    }
    mult
}

fn get_layers(data: String) -> ArrayBase<OwnedRepr<i64>, Dim<[usize; 3]>> {
    let data_len = data.len();
    let [max_layer, _, _] = get_index((data_len - 1) as i64);
    let mut index = 0 as usize;
    let mut layers =
        Array::<i64, _>::zeros((max_layer + 1 as usize, HEIGHT as usize, WIDTH as usize));
    while index < data_len {
        let [layer_index, y, x] = get_index(index as i64);
        let val = data[index..index + 1].parse::<i64>().unwrap();
        layers.slice_mut(s![layer_index, y, x]).fill(val);
        index += 1;
    }
    layers
}

fn get_index(index: i64) -> [usize; 3] {
    let layer_len = HEIGHT * WIDTH;
    let layer_index = index / layer_len;
    let in_layer = index % layer_len;
    let y = in_layer / WIDTH;
    let x = in_layer % WIDTH;
    [layer_index as usize, y as usize, x as usize]
}

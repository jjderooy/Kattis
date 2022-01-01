
use std::io::{self, BufRead};

struct Vertex{
    pub x: i32,
    pub y: i32
}

fn main() {

    let mut polygons: Vec< Vec<Vertex> > = Vec::new();
    get_polygons(&mut polygons);
    print_polygons(&polygons);
}

// Fills the provided vector with a vector of vertices from stdin
fn get_polygons(vec: &mut Vec< Vec<Vertex> >){

    let stdin = io::stdin();
    let mut handle = stdin.lock().lines();

    // Number of vertices for the current polygon
    let mut num_verts: i32 = handle.next().unwrap().unwrap().parse::<i32>().unwrap();

    // Input is terminated by a zero vertex case
    while num_verts != 0{

        let mut polygon: Vec<Vertex> = Vec::new();

        // Get all the vertices for this polygon
        for _ in 0..num_verts{

            /*  
                Get iterator over the numbers in this line of input

                Note: Since .unwrap returns a values, we have to store
                it before we split it or else the value is dropped, and the
                iterator is pointing at something that doesn't exist. This
                is what causes the dreaded: "temporary value dropped while borrowed"
            */
            let line = handle.next().unwrap().unwrap();
            let mut line_iter = line.split_whitespace();

            let v = Vertex {
                x: line_iter.next().unwrap().parse::<i32>().unwrap(),
                y: line_iter.next().unwrap().parse::<i32>().unwrap(),
            };
            polygon.push(v);
        }

        vec.push(polygon);

        // Now handle's line is the num of verts of the next polygon
        num_verts = handle.next().unwrap().unwrap().parse::<i32>().unwrap();
    }
}

fn print_polygons(polygons: &Vec< Vec<Vertex> >){

    for poly in polygons{
        for vert in poly{
            print!("[{}, {}] ", vert.x, vert.y);
        }
        println!();
    }
}

fn compute_area(polygon: &Vec<Vertex>) -> i32{


    0
}
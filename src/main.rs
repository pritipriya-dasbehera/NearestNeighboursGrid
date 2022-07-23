// use std::intrinsics::powf64;

// use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::time::{self, Duration};
use rand::Rng;
use arr_macro::arr;
use ndarray::{Array, Array3};

const NUM_OF_PARTICLES: usize = 1000;
const NEAREST_NEIGHBOURS_REQ: usize = 3;
const MAX_DIST: f64 = 0.8660254037844387;
const GRID_SIZE: usize = 6;
const GRID_LEN: f64 = 1.0/6.0;
// const MAX_DIST: f64 = 1.0;

fn main() {
    let mut now = time::Instant::now();
    let zeropt =  Point{x:0.0,y:0.0,z:0.0};
    let mut points: [Point;NUM_OF_PARTICLES] = [zeropt;NUM_OF_PARTICLES];
    point_generator(&mut points);
    let t_point_create = now.elapsed();
    println!("The time taken to create {} particles is {} microseconds.",NUM_OF_PARTICLES,t_point_create.as_micros());

    now = time::Instant::now();
    write_points(&points);
    let t_point_write = now.elapsed();
    println!("The time taken to write {} particles is {} microseconds.",NUM_OF_PARTICLES, t_point_write.as_micros());

    // now = time::Instant::now();
    // let mut nearlist: [Vec<Node>;NUM_OF_PARTICLES] = arr![vec![Node{id:0, dist:MAX_DIST}].clone();100]; // IMPORTANT..constant used here 
    // let t_init = now.elapsed();
    // println!("The time taken to initialise {} particle nearlist is {} microseconds.",NUM_OF_PARTICLES,t_init.as_micros());
    // print_nearlist(&nearlist);

    now = time::Instant::now();
    let mut grid = Array::from_elem((GRID_SIZE,GRID_SIZE,GRID_SIZE), vec![0_usize]);
    create_grid(&points, &mut grid);
    let t_grid = now.elapsed();
    println!("The time taken to make grid of {} particles is {} microseconds.",NUM_OF_PARTICLES,t_grid.as_micros());

    write_grid(&grid);

    // now = time::Instant::now();
    // brute_cal_nearest(&points, &mut nearlist);
    // let t_brute = now.elapsed();
    // println!("The time taken to brute calc {} particle nearlist is {} microseconds.",NUM_OF_PARTICLES,t_brute.as_micros());

    // now = time::Instant::now();
    // write_nearlist(&nearlist);
    // let elapsed_time = now.elapsed();
    // println!("The time taken to write {} particle nearlist is {} microseconds.",NUM_OF_PARTICLES,elapsed_time.as_micros());
    // print_nearlist(&nearlist);

    // write_time(t_point_create, t_point_write, t_init, t_brute);
}

// #[derive(Debug)]
#[derive(Clone, Copy)]
struct Point{x: f64,y:f64,z:f64}

// #[derive(Debug)]
#[derive(Clone, Copy)]
struct Node{id: u32, dist: f64}

fn dist(p1:&Point ,p2:&Point)->f64{
    let mut dx = (p1.x -p2.x).abs();
    let ax: f64 = (dx / 0.5 ).floor();
    dx = ax + (1.0 - 2.0 * ax) * dx;

    let mut dy = (p1.y -p2.y).abs();
    let ay: f64 = {dy / 0.5 }.floor();
    dy = ay + (1.0 - 2.0 * ay) * dy;


    let mut dz = (p1.z -p2.z).abs();
    let az: f64 = {dz / 0.5 }.floor();
    dz = az + (1.0 - 2.0 * az) * dz;

    // let r = {dx.powf(2.0) + dy.powf(2.0) + dz.powf(2.0)};
    // print!("The distance is {r} with dx={dx},ax={ax} dy={dy},ay={ay} and dz={dz},az={az}\n");
    // print!("The distance between p1 {:?} and p2 {:?} is {r} with dx={dx} dy={dy} and dz={dz}\n",&p1,&p2);
    dx.powf(2.0) + dy.powf(2.0) + dz.powf(2.0)
}

fn point_generator(points: &mut [Point]){

    // let mut file = File::create("points.csv").expect("Something went wrong in file creation");
    // let mut file = File::options().append(true).open("points.csv").expect("Something went wrong in file creation");
    for (_i,element) in points.iter_mut().enumerate(){
        element.x = rand::thread_rng().gen::<f64>();
        element.y = rand::thread_rng().gen::<f64>();
        element.z = rand::thread_rng().gen::<f64>();
        // print!("The point {i} is {:?}\n",element);
        // let mut data = String::new();
        // data.push_str(&i.to_string());
        // data.push_str(",");
        // data.push_str(&element.x.to_string());
        // data.push_str(",");
        // data.push_str(&element.y.to_string());
        // data.push_str(",");
        // data.push_str(&element.z.to_string());
        // data.push_str("\n");
        // file.write_all(data.as_bytes()).expect("unable to write");
    }

}

// fn _print_nearlist(nearlist:&[Vec<Node>;NUM_OF_PARTICLES]){
//     for (i,element) in nearlist.iter().enumerate(){
//         println!("The vector {i} is {:?}",element)
//     }
// }

fn write_points(points: &[Point]){
    let mut file = File::create("points.csv").expect("Something went wrong in file creation");
    // let mut file = File::options().append(true).open("points.csv").expect("Something went wrong in file creation");
    file.write_all(b"id,x,y,z\n").expect("unable to write");
    for (i,element) in points.iter().enumerate(){
        let mut data = String::new();
        data.push_str(&i.to_string());
        data.push(',');
        data.push_str(&element.x.to_string());
        data.push(',');
        data.push_str(&element.y.to_string());
        data.push(',');
        data.push_str(&element.z.to_string());
        data.push('\n');
        file.write_all(data.as_bytes()).expect("unable to write");
        
        // let row = concat!(i.to_string,b",",element.x.to_be_bytes);
        // let str = i.to_be_bytes()+",".as_bytes()+element.x.to_be_bytes();
    }
}

fn write_nearlist(nearlist: &[Vec<Node>;NUM_OF_PARTICLES]){
    let mut file = File::create("nearlist.csv").expect("Something went wrong in file creation");
    // let mut file = File::options().append(true).open("nearlist.csv").expect("Something went wrong in file creation");
    for (i, _element) in nearlist.iter().enumerate().take(NUM_OF_PARTICLES)
    {
        let mut data = String::new();
        for (j,element) in nearlist[i].iter().enumerate(){
            if j==0{data.push_str(&i.to_string());continue};
            if j==NEAREST_NEIGHBOURS_REQ+1{break;};
            data.push(',');
            data.push_str(&element.id.to_string());
        }
        data.push('\n');
        file.write_all(data.as_bytes()).expect("unable to write");
    }
}

fn write_time(t_point_create:Duration,t_point_write:Duration,t_init:Duration,t_brute:Duration){
    let mut file = File::options().append(true).open("time.csv").expect("Something went wrong in file creation");
    let mut data = String::new();
    data.push_str(&NUM_OF_PARTICLES.to_string());
    data.push(',');
    data.push_str(&t_point_create.as_micros().to_string());
    data.push(',');
    data.push_str(&t_point_write.as_micros().to_string());
    data.push(',');
    data.push_str(&t_init.as_micros().to_string());
    data.push(',');
    data.push_str(&t_brute.as_micros().to_string());
    data.push('\n');
    file.write_all(data.as_bytes()).expect("unable to write");

}

fn create_grid(points: &[Point], grid:&mut Array3<Vec<usize>>){
    for (i,point) in points.iter().enumerate(){
        let x = unsafe { (point.x/GRID_LEN).floor().to_int_unchecked()};
        let y = unsafe { (point.y/GRID_LEN).floor().to_int_unchecked()};
        let z = unsafe { (point.z/GRID_LEN).floor().to_int_unchecked()};
        grid[[x,y,z]][0] += 1;
        let index = grid[[x,y,z]][0];
        grid[[x,y,z]].insert(index, i);
    }
}

fn write_grid(grid:&Array3<Vec<usize>>){
    let mut file = File::create("grid.csv").expect("Something went wrong in file creation");
    for cell in grid.iter(){
        let mut data = String::new();
        for id in cell.iter(){
            data.push_str(&id.to_string());
            data.push(',')
        }
        data.push('\n');
        file.write_all(data.as_bytes()).expect("Couldnt write the points");
    }
}

// fn write_grid(grid:&Array3<Vec<usize>>){
//     let mut file = File::create("nearlist.csv").expect("Something went wrong in file creation");
//     {0..GRID_SIZE}.for_each(|i|
//         {0..GRID_SIZE}.for_each(|j|
//             {0..GRID_SIZE}.for_each(|k|{
//                 file.write_all();
//             }
//             )))
// }

fn brute_cal_nearest(points: &[Point], nearlist: &mut [Vec<Node>;NUM_OF_PARTICLES]){
    // let mut file = File::create("nearlist.csv").expect("Something went wrong in file creation");
    // let mut file = File::options().append(true).open("nearlist.csv").expect("Something went wrong in file creation");
    {0..NUM_OF_PARTICLES}.for_each(|i|
    {
        for j in 0..NUM_OF_PARTICLES
        {
            if i == j
            {
                continue;
            }
            let d = dist(&points[i],&points[j]);
            // print!("The distance between point {i} and point {j} is {d}\n");
            if d<nearlist[i][0].dist{
                // print!("Inserting node for {i}\t");
                let mut k = 1;
                while nearlist[i].len()>k && nearlist[i][k].dist < d{
                    k+=1;
                }
                nearlist[i].insert(k, Node{id:j as u32,dist:d});

                if nearlist[i].len() > NEAREST_NEIGHBOURS_REQ+1 {
                    nearlist[i][0].dist = nearlist[i][NEAREST_NEIGHBOURS_REQ].dist;
                    nearlist[i].remove(NEAREST_NEIGHBOURS_REQ + 1);
                }
            }
        }
        // let mut data = String::new();
        // for (j,element) in nearlist[i].iter().enumerate(){
        //     if j==0{data.push_str(&i.to_string());continue};
        //     if j==NEAREST_NEIGHBOURS_REQ+1{break;};
        //     data.push_str(",");
        //     data.push_str(&element.id.to_string());
        // }
        // data.push_str("\n");
        // file.write_all(data.as_bytes()).expect("unable to write");
    })
}


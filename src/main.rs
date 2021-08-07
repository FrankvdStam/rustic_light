#![allow(unused)]

mod gpu;
use crate::gpu::{Gpu, RgbFusionMode, RgbFusionSpeed};

mod color;
mod motherboard;

use crate::color::Color;
use crate::motherboard::Motherboard;
use std::thread::sleep;
use std::time::{Duration, SystemTime};


const MAX_MAP_GRAPH: u32 = 767;

const R_OFFSET: u32 = 0;
const G_OFFSET: u32 = 255;
const B_OFFSET: u32 = 511;


/*
		We can generate 3 phases of this graph:
		(y: r/g/b)
		255 /|\   |    |
		   / | \  |    |
		  /  |  \ |    |
		0/   |   \|____|
	    0   255  511  767 (x: potmeter position)
		if(x < 256)
		{
			y = x;
		}
		else if (x > 255 && x < 512)
		{
			y = 256 - x;
		}
		else{
			y = 0;
		}
		We offset each color to get 3 phases, thats the rgb values. */



fn map_graph_color(mut position: u32, offset: u32) -> u8
{
    //println!("before {}", position);
    //loop pos around 0-767
    position = (position + offset) % MAX_MAP_GRAPH;
    //println!("after {}", position);

    //generate the graph
    if position <= 255
    {
        return position as u8;
    }

    if position >= 256 && position <= 511
    {
        return (255 - (position - 256)) as u8;
    }

    return 0;
}

fn get_color_from_graph(mut millis: u128) -> Color
{
    //scale
    millis = millis / 100;

    return Color::new(
        map_graph_color(millis as u32, R_OFFSET),
        map_graph_color(millis as u32, G_OFFSET),
        map_graph_color(millis as u32, B_OFFSET)
    );
}


fn main()
{
    let start = SystemTime::now();

    let gpu = Gpu::new();
    //gpu.set_color(Color::new(0, 0, 255));
    gpu.set_mode(RgbFusionMode::Static, RgbFusionSpeed::Fastest);

    let motherboard = Motherboard::new();
    //motherboard.set_color(Color::new(0, 255, 0));

    loop {
        sleep(Duration::from_millis(100));
        let position = SystemTime::now().duration_since(start).unwrap().as_millis();
        let color = get_color_from_graph(position);
        //println!("{}", color);
        gpu.set_color(color);
        motherboard.set_color(color);
        //motherboard.set_color(color);
    }
}


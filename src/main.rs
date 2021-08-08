mod color;
mod z390;
mod rtx2080;

use rtx2080::Rtx2080;
use crate::color::{Color, RgbDevice};
use crate::z390::{Motherboard, Z390};
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

    let z390 = Z390::new();
    let rtx2080 = Rtx2080::new();

    let mut rgb_devices: Vec<Box<dyn RgbDevice>> = Vec::new();
    rgb_devices.push(Box::new(rtx2080));




    loop
    {
        let position = SystemTime::now().duration_since(start).unwrap().as_millis();
        let color = get_color_from_graph(position);
        //println!("{}", color);
        //gpu.set_color(color);
        //motherboard.set_color(color);

        for d in rgb_devices.iter_mut()
        {
            d.set_color(color);
            d.display();
        }


        //motherboard.set_color(color);
        sleep(Duration::from_millis(100));
    }
}


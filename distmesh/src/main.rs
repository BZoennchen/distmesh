use delaunator::{Point, triangulate};

/*fn main() {
    let points = vec![
        Point { x: 0., y: 0. },
        Point { x: 1., y: 0. },
        Point { x: 2., y: 1. },
        Point { x: 0., y: 1. },
    ];
    
    let result = triangulate(&points);
    println!("{:?}", result.triangles); // [0, 2, 1, 0, 3, 2]
    println!("{:?}", result.halfedges);
    println!("{:?}", result.hull); 
}*/


use nannou::prelude::*;
use distmesh::prelude::*;
use distmesh::tikz::to_tikz_string;
use std::fs::File;
use std::io::Write;

const SIZE: u32 = 800;
const POINT_SIZE: f32 = 5.0;
const BORDER_WIDTH: f64 = 100.0;
const N: usize = 600;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    distmesh: DistMesh,
}

impl Model {    
    fn update(&mut self) {
        self.distmesh.update(0.1);

        println!("quality: {}", avg_quality(
            &self.distmesh.points, 
            &self.distmesh.triangulation.triangles)
        );
    }
}

trait TraitPoint {
    fn p2d(self: &Self) -> Point2;
}

impl TraitPoint for Point {

    fn p2d(self: &Self) -> Point2 {
        pt2(self.x as f32, self.y as f32)
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .title(app.exe_name().unwrap())
        .size(SIZE, SIZE)
        .view(view)
        .build()
        .unwrap();

    let half_size = (SIZE-100) as f64 / 2.0;
    let builder: DistMeshBuilder = DistMeshBuilder::new(N)
        .x1(-half_size).x2(half_size)
        .y1(-half_size).y2(half_size)
        .virtual_edges()
        .break_edges()
        .bosson()
        //.edge_len_fn(|p|{1.0 + Point {x: 0.0, y: 0.0}.distance(p) / 30.0})
        .dist_fn(Box::new(Ring::new(Point {x: 0.0, y: 0.0}, 100.0, 300.0)));
        //.dist_fn(Box::new(Circle::new(Point {x: 0.0, y: 0.0}, 300.0)));
        /*.dist_fn(Box::new(Rect::new(Point {x: 0.0, y: 0.0}, 500.0, 500.0)))
        .add_fixpoint(Point {x: 250.0, y: 250.0})
        .add_fixpoint(Point {x: -250.0, y: 250.0})
        .add_fixpoint(Point {x: 250.0, y: -250.0})
        .add_fixpoint(Point {x: -250.0, y: -250.0}); */

    let distmesh = builder.build();
    
    Model { _window, distmesh}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.update();
    if app.elapsed_frames() % 100 == 0 {
        //println!("{}", &to_tikz_string(&model.distmesh));

        // Create a new file (or truncate if it exists)
        //let mut file = File::create("output.tex").expect("error!");
        
        // Write the string to the file
        //file.write_all(&to_tikz_string(&model.distmesh).as_bytes()).expect("error!!");
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    let distmesh = &model.distmesh;

    for index in 0..distmesh.triangulation.triangles.len()  {
        if index % 3 == 2 {
            let tri_index = distmesh.triangulation.triangles[index-2];
            let p1: Point2 = distmesh.points[tri_index].p2d();
            
            let tri_index = distmesh.triangulation.triangles[index-1];
            let p2: Point2 = distmesh.points[tri_index].p2d();
            
            let tri_index = distmesh.triangulation.triangles[index];
            let p3: Point2 = distmesh.points[tri_index].p2d();

            draw.tri().points(p1, p2, p3).stroke_weight(1.0).color(WHITE).stroke_color(BLACK);
        }
    }

    for p in distmesh.points.iter() {
        let p1 = p.p2d();
        draw.ellipse().xy(p1).w(POINT_SIZE).h(POINT_SIZE).color(BLACK);
    }
    
    draw.to_frame(app, &frame).unwrap();
}

/*use distmesh::prelude::*;

fn main() {
    let npoints = 300;
    let niterations = 1000;

    println!("build distmesh");
    let mut builder = DistMeshBuilder::new(npoints)      
        .x1(-350.0).x2(350.0)
        .y1(-350.0).y2(350.0)
        //.dist_fn(Box::new(Circle::new(Point {x: 0.0, y: 0.0}, 300.0)));
        .dist_fn(Box::new(Rect::new(Point {x: 0.0, y: 0.0}, 300.0, 300.0)));
        //.add_fixpoint(Point {x: 150.0, y: 150.0})
        //.add_fixpoint(Point {x: -150.0, y: 150.0})
        //.add_fixpoint(Point {x: 150.0, y: -150.0})
        //.add_fixpoint(Point {x: -150.0, y: -150.0});

    let m = 20;
    for i in 0..m {
        builder = builder.add_fixpoint(Point {x: -150.0 + (i/(m-1)) as f64 * 300.0, y: -150.0});
        builder = builder.add_fixpoint(Point {x: -150.0 + (i/(m-1)) as f64 * 300.0, y: 150.0});
        builder = builder.add_fixpoint(Point {x: -150.0, y: -150.0 + (i/(m-1)) as f64 * 300.0});
        builder = builder.add_fixpoint(Point {x: 150.0, y: -150.0 + (i/(m-1)) as f64 * 300.0});
    }

    let mut distmesh = builder.build();
    println!("finish building distmesh");

    for i in 0..niterations {
        distmesh.update(DELTA_T);
        println!("step {}, quality: {}", (i+1), quality(&distmesh.points, &distmesh.triangulation.triangles));
    }
}*/
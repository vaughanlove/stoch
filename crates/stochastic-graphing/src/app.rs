use eframe::egui::plot::{Plot, Line, PlotPoints, PlotPoint};
use stochastic_processes::processes::{Process, TimeSeries, TimePoint}; 
use stochastic_processes::rvector::{RandomVector, Sample};
use stochastic_processes::mappings::{martingale_strat};
use rand_distr::{Uniform};


pub struct GraphApp {
    // todo: add serialization of lines?
    lines: Vec<Process<TimePoint>>
}

pub trait PlotSeries {
    /// turns &mut reference to self into PlotPoints for visualizing with egui.
    /// A &mut reference is present when Process<TimePoint> objects are being referenced from a vector. 
    /// 
    /// # Arguments
    /// 
    /// *'&mut self'
    /// 
    fn mut_into(&mut self) -> PlotPoints;
}

impl Default for GraphApp {
    fn default() -> Self {
        // generate 5 paths of 200 timepoints. 
        //let p = <Process<TimePoint> as BrownianMotion>::many_new(200, 5);

        // deref the Box<> paths.
        // let mut lines = Vec::new();
        // for x in p {
        //     lines.push(*x) 
        // }

        let uni = Uniform::new(0, 2);
        let mut rng = rand::thread_rng();
        let n = 10;

        let rv = RandomVector::new(uni, &mut rng, n);
        let a = RandomVector::<f64>::from(rv);
        let p = Process::run_sim(&a, martingale_strat);
        let lines = vec![p];


        Self {
            lines
        }
    }
}

impl GraphApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for GraphApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { lines } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
            });
            if ui.button("Increment").clicked() {
                // let n = 200;
                // let dist = StandardNormal;
                // let mut rng = rand::thread_rng();
            
                // let rv = RandomVector::new(dist, &mut rng, n);
                // let p = Process::run_sim(&rv, sum);
                let uni = Uniform::new(0, 2);
                let mut rng = rand::thread_rng();
                let n = 10;
        
                let rv = RandomVector::<f64>::from(RandomVector::new(uni, &mut rng, n));
                let p = Process::run_sim(&rv, martingale_strat);

                lines.push(p);
                let m = lines.len();
                println!("{m:}");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {  
            {
                Plot::new("plot").view_aspect(2.0).show(ui, |plot_ui| {
                    for i in 0..lines.len(){
                        let m: PlotPoints = lines[i].mut_into();
                        let l = Line::new(m);   
                        plot_ui.line(l)}
                });
            }        
            egui::warn_if_debug_build(ui);
        });
    }
}


impl PlotSeries for Process<TimePoint> {
    fn mut_into(&mut self) -> PlotPoints {
        let mut v = Vec::new();
        for p in self.data.as_slice(){
            v.push( PlotPoint {x: p.t, y: p.y})
        }
        PlotPoints::Owned(v)
    }
    
}
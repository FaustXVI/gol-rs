use std::prelude::v1::*;
use gtk::prelude::*;
use gtk::{Window, Builder, DrawingArea};
use cairo::Context;
use std::sync::{Arc, RwLock};
use std::cell::RefCell;
use std::ops::Deref;
use crate::game_of_life::api_types::{Grid, Column, Row};

type ThreadSafeRef<T> = Arc<RwLock<RefCell<T>>>;

type GridPrinter = dyn Fn(Box<dyn Grid>);

pub fn build_ui() -> Box<GridPrinter> {
    let glade_src = include_str!("builder.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: Window = builder.get_object("window1").expect("No window");
    window.connect_destroy(move |_| {
        gtk::main_quit();
    });
    window.show_all();

    let area: DrawingArea = builder.get_object("drawingArea").expect("No drawingArea");

    let grid_reference: ThreadSafeRef<Option<Box<dyn Grid>>> = Arc::new(RwLock::new(RefCell::new(None)));
    let display_grid_reference = grid_reference.clone();
    area.connect_draw(move |a, cr| {
        let guard = display_grid_reference.read().unwrap();
        let may_be_grid = (*guard).borrow();
        if let Some(grid) = may_be_grid.deref() {
            let h = a.get_allocated_height();
            let w = a.get_allocated_width();
            let size = grid.size();
            cr.scale(w as f64 / size.width as f64, h as f64 / size.height as f64);
            draw_grid(cr, grid.deref());
        }
        Inhibit(false)
    });

    Box::new(move |new_grid: Box<dyn Grid>| {
        { grid_reference.write().unwrap().replace(Some(new_grid)); }
        area.queue_draw();
    })
}

fn draw_grid(cr: &Context, grid: &dyn Grid) {
    let size = grid.size();
    for row in 0..size.height {
        for column in 0..size.width {
            if grid.has_cell_at(Row(row), Column(column)) {
                cr.set_source_rgb(0f64, 0f64, 0f64);
            } else {
                cr.set_source_rgb(1f64, 1f64, 1f64);
            }
            cr.rectangle(column as f64, row as f64, 1.0, 1.0);
            cr.fill();
        }
    }
}
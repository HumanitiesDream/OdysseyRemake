//! The crust sags under ice, lifts a forebulge, and rebounds when the ice goes.
use odyssey_core::build::*;
use odyssey_core::planet::*;
use odyssey_core::sim::Parallel;

fn dome(b: &mut Built, centre: usize, radius_km: f32, thick_m: f32, on: bool) {
    let g = b.sim.world.grid.clone();
    let c = g.pos[centre];
    let ph = b.env.ph.clone();
    for i in 0..g.len() {
        let p = g.pos[i];
        let d = (c[0] * p[0] + c[1] * p[1] + c[2] * p[2]).clamp(-1.0, 1.0).acos() * 6371.0;
        if d < radius_km {
            for k in 0..12 {
                let v = if on { thick_m * RHO_ICE / 12.0 } else { 0.0 };
                b.sim.world.stock_mut(ph.w_ice[k])[i] = v;
            }
        }
    }
}

fn sag(b: &Built, i: usize) -> f32 { b.sim.world.stock(b.env.ph.defl)[i] - DEFL_OFFSET }

#[test]
fn sag_forebulge_and_rebound() {
    let mut s = Setup::today(48);
    s.only = Some(vec!["crust".into()]);
    let mut b = build(&s).unwrap();
    b.sim.parallel = Parallel::Off;
    // an ice dome 3000 m thick, 1500 km wide, on a quiet part of the map
    let centre = (0..b.sim.world.grid.len()).find(|&i| b.env.p.lat_deg[i].abs() < 5.0 && b.sim.world.grid.lon[i].abs() < 0.1).unwrap();
    // zero the base load at this centre by building the crust with the dome already counted as absent:
    dome(&mut b, centre, 750.0, 3000.0, true);
    b.sim.run_years(30_000);
    let w_centre = sag(&b, centre);
    let local = 3000.0 * 917.0 / 3300.0;
    println!("centre sag {w_centre:.0} m, local isostasy {local:.0} m");
    assert!(w_centre > 0.85 * local && w_centre < 1.05 * local, "sag {w_centre}");
    // forebulge: somewhere outside the dome the crust is lifted
    let g = b.sim.world.grid.clone();
    let c = g.pos[centre];
    let lifted = (0..g.len()).filter(|&i| {
        let p = g.pos[i];
        let d = (c[0] * p[0] + c[1] * p[1] + c[2] * p[2]).clamp(-1.0, 1.0).acos() * 6371.0;
        d > 800.0 && d < 1500.0 && sag(&b, i) < -5.0
    }).count();
    println!("forebulge tiles {lifted}");
    assert!(lifted > 0, "no forebulge");
    // remove the ice: after one relaxation time about 37 percent of the sag is left
    dome(&mut b, centre, 750.0, 0.0, false);
    b.sim.run_years(3_000);
    let left = sag(&b, centre) / w_centre;
    println!("after 3000 years {left:.2} of the sag is left");
    assert!(left > 0.30 && left < 0.45, "left {left}");
}

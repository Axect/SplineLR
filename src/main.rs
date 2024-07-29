use std::f64::consts::PI;

use peroxide::fuga::*;

type PTS = (Vec<f64>, Vec<f64>);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_epoch = 100.0;
    let init_lr   = 1f64;
    let min_lr    = 1e-3;

    let splr = SplineLR { init_lr: init_lr.ln(), min_lr: min_lr.ln(), N: max_epoch };
    let domain = linspace(0, max_epoch, 200);
    for i in 1 ..= 3 {
        let (cs_plus, cs_minus) = splr.gen_splines(i)?;
        let y_plus = cs_plus.eval_vec(&domain);
        let y_minus = cs_minus.eval_vec(&domain);

        let y_plus = y_plus.fmap(|x| x.exp());
        let y_minus = y_minus.fmap(|x| x.exp());

        let mut plt = Plot2D::new();
        plt
            .set_domain(domain.clone())
            .insert_image(y_plus)
            .insert_image(y_minus)
            .set_xlabel("Epoch")
            .set_ylabel("Learning Rate")
            .set_yscale(PlotScale::Log)
            .set_style(PlotStyle::Nature)
            .set_dpi(600)
            .tight_layout()
            .set_path(&format!("spline{}.png", i))
            .savefig()?;
    }

    Ok(())
}

#[allow(non_snake_case)]
pub struct SplineLR {
    init_lr: f64,
    min_lr: f64,
    N: f64,
}

impl SplineLR {
    fn delta_pt(&self) -> (f64, f64) {
        let theta = self.N.atan2(self.init_lr - self.min_lr);
        let alpha = theta.min(PI / 2.0 - theta) * 0.99;
        let l = ((self.init_lr - self.min_lr).powi(2) + self.N.powi(2)).sqrt() / 4f64;
    
        (l * alpha.tan() * theta.sin(), l * alpha.tan() * theta.cos())
    }

    fn gen_pts(&self, i: usize) -> (PTS, PTS) {
        assert!((1..=3).contains(&i));

        let (delta_x, delta_y) = self.delta_pt();

        let x = linspace(0, self.N, 5);
        let y = linspace(self.init_lr, self.min_lr, 5);

        let mut x_plus = x.clone();
        let mut y_plus = y.clone();
        let mut x_minus = x.clone();
        let mut y_minus = y.clone();

        x_plus[i] += delta_x;
        y_plus[i] += delta_y;
        x_minus[i] -= delta_x;
        y_minus[i] -= delta_y;

        ((x_plus, y_plus), (x_minus, y_minus))
    }

    fn gen_splines(&self, i: usize) -> Result<(CubicHermiteSpline, CubicHermiteSpline), Box<dyn std::error::Error>> {
        let (pts_plus, pts_minus) = self.gen_pts(i);

        let cs_plus = cubic_hermite_spline(&pts_plus.0, &pts_plus.1, Quadratic)?;
        let cs_minus = cubic_hermite_spline(&pts_minus.0, &pts_minus.1, Quadratic)?;

        Ok((cs_plus, cs_minus))
    }
}

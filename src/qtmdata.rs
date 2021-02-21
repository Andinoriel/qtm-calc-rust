#[allow(dead_code)]
use pyo3::prelude::*;

use crate::qtmcore::Qtm;
use crate::utility::clamp_xsize;

#[pyclass]
pub struct QtmData {}

#[pymethods]
impl QtmData {
    #[staticmethod]
    fn calc_avg_queue(qtm: &Qtm) -> f64 {
        let mut i = 1;
        let mut res = 0.;

        for item in &qtm.final_states()[qtm.channel_count + 1..] {
            res += i as f64 * *item;
            i = clamp_xsize::<usize>(i + 1, 1, qtm.queue_size);
        }

        res
    }
    #[staticmethod]
    fn calc_ete(qtm: &Qtm) -> f64 {
        let mut i = 0;
        let mut res = 0.;

        for item in qtm.final_states().iter() {
            res += i as f64 * *item;
            i = clamp_xsize::<usize>(i + 1, 0, qtm.channel_count);
        }

        res / qtm.channel_count as f64
    }
    #[staticmethod]
    fn calc_avg_time_queue(qtm: &Qtm) -> f64 {
        QtmData::calc_avg_queue(qtm) / qtm.channel_count as f64 * qtm.mu
    }
    #[staticmethod]
    fn calc_perc_served_req(qtm: &Qtm) -> f64 {
        1. - qtm.final_states().last().unwrap().clone()
    }
    #[staticmethod]
    fn calc_avg_count_served_req(qtm: &Qtm) -> f64 {
        let mut i = 0;
        let mut res = 0.;

        for item in qtm.final_states().iter() {
            res += *item * i as f64;
            if i < qtm.channel_count + 1 {
                i += 1;
            }
        }

        res
    }
    #[staticmethod]
    fn calc_avg_count_req(qtm: &Qtm) -> f64 {
        let mut i = 0;
        let mut res = 0.;

        for item in qtm.final_states().iter() {
            res += *item * i as f64;
            i += 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::{Qtm, QtmData};

    #[test]
    fn data_check_avg_queue() {
        let comp = 0.2857142857142857;

        let mut qtm = Qtm::new(2, 1, 10., 5., 0., -1);
        qtm.calc_final_states();
        let ret = QtmData::calc_avg_queue(&qtm);

        assert_eq!(comp, ret);
    }
    #[test]
    fn data_check_ete() {
        let comp = 0.71428571428571419;

        let mut qtm = Qtm::new(2, 1, 10., 5., 0., -1);
        qtm.calc_final_states();
        let ret = QtmData::calc_ete(&qtm);

        assert_eq!(comp, ret);
    }
    #[test]
    fn data_check_avg_time_queue() {
        let comp = 0.71428571428571419;

        let mut qtm = Qtm::new(2, 1, 10., 5., 0., -1);
        qtm.calc_final_states();
        let ret = QtmData::calc_avg_time_queue(&qtm);

        assert_eq!(comp, ret);
    }
    #[test]
    fn data_check_perc_served_req() {
        let comp = 0.7142857142857143;

        let mut qtm = Qtm::new(2, 1, 10., 5., 0., -1);
        qtm.calc_final_states();
        let ret = QtmData::calc_perc_served_req(&qtm);

        assert_eq!(comp, ret);
    }
    #[test]
    fn data_check_avg_count_served_req() {
        let comp = 1.7142857142857142;

        let mut qtm = Qtm::new(2, 1, 10., 5., 0., -1);
        qtm.calc_final_states();
        let ret = QtmData::calc_avg_count_served_req(&qtm);

        assert_eq!(comp, ret);
    }
    #[test]
    fn data_check_avg_count_req() {
        let comp = 1.7142857142857142;

        let mut qtm = Qtm::new(2, 1, 10., 5., 0., -1);
        qtm.calc_final_states();
        let ret = QtmData::calc_avg_count_req(&qtm);

        assert_eq!(comp, ret);
    }
}

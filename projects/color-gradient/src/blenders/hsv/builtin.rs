use super::*;

/// standard hsv color maps
impl HsvGradient {
    pub fn standard(min: f32, max: f32) -> HsvGradient {
        let mut grad = HsvGradient::new(0.0, 360.0);
        grad.insert_hue(0.0, 0.0);
        grad.insert_hue(60.0, 60.0);
        grad.insert_hue(120.0, 120.0);
        grad.insert_hue(180.0, 180.0);
        grad.insert_hue(240.0, 240.0);
        grad.insert_hue(300.0, 300.0);
        grad.insert_hue(360.0, 360.0);
        grad.rescale(min, max);
        grad
    }
}

/// matlab color maps
impl HsvGradient {
    pub fn parula(min: f32, max: f32) -> HsvGradient {
        let mut grad = HsvGradient::new(0.0, 430.00);
        grad.insert_hue(0.00, 251.08);
        grad.insert_saturation(0.00, 77.38);
        grad.insert_brightness(0.00, 65.88);
        grad.insert_hue(43.00, 241.83);
        grad.insert_saturation(43.00, 71.30);
        grad.insert_brightness(43.00, 90.20);
        grad.insert_hue(86.00, 229.30);
        grad.insert_saturation(86.00, 73.12);
        grad.insert_brightness(86.00, 99.22);
        grad.insert_hue(129.00, 211.66);
        grad.insert_saturation(129.00, 81.56);
        grad.insert_brightness(129.00, 95.69);
        grad.insert_hue(172.00, 196.31);
        grad.insert_saturation(172.00, 87.44);
        grad.insert_brightness(172.00, 87.45);
        grad.insert_hue(215.00, 178.62);
        grad.insert_saturation(215.00, 91.58);
        grad.insert_brightness(215.00, 74.51);
        grad.insert_hue(258.00, 148.40);
        grad.insert_saturation(258.00, 64.53);
        grad.insert_brightness(258.00, 79.61);
        grad.insert_hue(301.00, 79.70);
        grad.insert_saturation(301.00, 66.67);
        grad.insert_brightness(301.00, 78.82);
        grad.insert_hue(344.00, 44.52);
        grad.insert_saturation(344.00, 79.49);
        grad.insert_brightness(344.00, 91.76);
        grad.insert_hue(387.00, 49.12);
        grad.insert_saturation(387.00, 81.60);
        grad.insert_brightness(387.00, 98.04);
        grad.insert_hue(430.00, 60.52);
        grad.insert_saturation(430.00, 91.63);
        grad.insert_brightness(430.00, 98.43);
        grad.rescale(min, max);
        grad
    }
    pub fn jet(min: f32, max: f32) -> HsvGradient {
        let mut grad = HsvGradient::new(0.0, 430.00);
        grad.insert_hue(0.00, 240.00);
        grad.insert_saturation(0.00, 100.00);
        grad.insert_brightness(0.00, 51.37);
        grad.insert_hue(43.00, 240.00);
        grad.insert_saturation(43.00, 100.00);
        grad.insert_brightness(43.00, 90.59);
        grad.insert_hue(86.00, 222.12);
        grad.insert_saturation(86.00, 100.00);
        grad.insert_brightness(86.00, 100.00);
        grad.insert_hue(129.00, 197.88);
        grad.insert_saturation(129.00, 100.00);
        grad.insert_brightness(129.00, 100.00);
        grad.insert_hue(172.00, 172.60);
        grad.insert_saturation(172.00, 89.02);
        grad.insert_brightness(172.00, 100.00);
        grad.insert_hue(215.00, 120.00);
        grad.insert_saturation(215.00, 49.80);
        grad.insert_brightness(215.00, 100.00);
        grad.insert_hue(258.00, 66.23);
        grad.insert_saturation(258.00, 90.59);
        grad.insert_brightness(258.00, 100.00);
        grad.insert_hue(301.00, 42.12);
        grad.insert_saturation(301.00, 100.00);
        grad.insert_brightness(301.00, 100.00);
        grad.insert_hue(344.00, 17.88);
        grad.insert_saturation(344.00, 100.00);
        grad.insert_brightness(344.00, 100.00);
        grad.insert_hue(387.00, 0.00);
        grad.insert_saturation(387.00, 100.00);
        grad.insert_brightness(387.00, 89.02);
        grad.insert_hue(430.00, 0.00);
        grad.insert_saturation(430.00, 100.00);
        grad.insert_brightness(430.00, 50.20);
        grad.rescale(min, max);
        grad
    }
    pub fn turbo(min: f32, max: f32) -> HsvGradient {
        let mut grad = HsvGradient::new(0.0, 430.00);
        grad.insert_hue(0.00, 283.90);
        grad.insert_saturation(0.00, 69.49);
        grad.insert_brightness(0.00, 23.14);
        grad.insert_hue(43.00, 231.04);
        grad.insert_saturation(43.00, 66.01);
        grad.insert_brightness(43.00, 79.61);
        grad.insert_hue(86.00, 212.04);
        grad.insert_saturation(86.00, 74.90);
        grad.insert_brightness(86.00, 100.00);
        grad.insert_hue(129.00, 177.45);
        grad.insert_saturation(129.00, 88.26);
        grad.insert_brightness(129.00, 83.53);
        grad.insert_hue(172.00, 140.90);
        grad.insert_saturation(172.00, 71.77);
        grad.insert_brightness(172.00, 97.25);
        grad.insert_hue(215.00, 88.75);
        grad.insert_saturation(215.00, 75.89);
        grad.insert_brightness(215.00, 99.22);
        grad.insert_hue(258.00, 58.59);
        grad.insert_saturation(258.00, 75.56);
        grad.insert_brightness(258.00, 88.24);
        grad.insert_hue(301.00, 34.41);
        grad.insert_saturation(301.00, 80.31);
        grad.insert_brightness(301.00, 99.61);
        grad.insert_hue(344.00, 19.73);
        grad.insert_saturation(344.00, 92.50);
        grad.insert_brightness(344.00, 94.12);
        grad.insert_hue(387.00, 10.62);
        grad.insert_saturation(387.00, 98.46);
        grad.insert_brightness(387.00, 76.47);
        grad.insert_hue(430.00, 0.50);
        grad.insert_saturation(430.00, 97.54);
        grad.insert_brightness(430.00, 47.84);
        grad.rescale(min, max);
        grad
    }
    pub fn hot(min: f32, max: f32) -> HsvGradient {
        let mut grad = HsvGradient::new(0.0, 420.00);
        grad.insert_hue(0.00, 0.00);
        grad.insert_saturation(0.00, 100.00);
        grad.insert_brightness(0.00, 1.18);
        grad.insert_hue(42.00, 0.00);
        grad.insert_saturation(42.00, 100.00);
        grad.insert_brightness(42.00, 25.88);
        grad.insert_hue(84.00, 0.00);
        grad.insert_saturation(84.00, 100.00);
        grad.insert_brightness(84.00, 52.16);
        grad.insert_hue(126.00, 0.00);
        grad.insert_saturation(126.00, 100.00);
        grad.insert_brightness(126.00, 78.04);
        grad.insert_hue(168.00, 2.59);
        grad.insert_saturation(168.00, 100.00);
        grad.insert_brightness(168.00, 100.00);
        grad.insert_hue(210.00, 18.12);
        grad.insert_saturation(210.00, 100.00);
        grad.insert_brightness(210.00, 100.00);
        grad.insert_hue(252.00, 33.65);
        grad.insert_saturation(252.00, 100.00);
        grad.insert_brightness(252.00, 100.00);
        grad.insert_hue(294.00, 49.41);
        grad.insert_saturation(294.00, 100.00);
        grad.insert_brightness(294.00, 100.00);
        grad.insert_hue(336.00, 60.00);
        grad.insert_saturation(336.00, 87.45);
        grad.insert_brightness(336.00, 100.00);
        grad.insert_hue(378.00, 60.00);
        grad.insert_saturation(378.00, 48.63);
        grad.insert_brightness(378.00, 100.00);
        grad.insert_hue(420.00, 60.00);
        grad.insert_saturation(420.00, 9.41);
        grad.insert_brightness(420.00, 100.00);
        grad.rescale(min, max);
        grad
    }
    pub fn cool(min: f32, max: f32) -> HsvGradient {
        let mut grad = HsvGradient::new(0.0, 430.00);
        grad.insert_hue(0.00, 180.00);
        grad.insert_saturation(0.00, 100.00);
        grad.insert_brightness(0.00, 100.00);
        grad.insert_hue(43.00, 186.52);
        grad.insert_saturation(43.00, 90.20);
        grad.insert_brightness(43.00, 100.00);
        grad.insert_hue(86.00, 194.63);
        grad.insert_saturation(86.00, 80.39);
        grad.insert_brightness(86.00, 100.00);
        grad.insert_hue(129.00, 205.47);
        grad.insert_saturation(129.00, 70.20);
        grad.insert_brightness(129.00, 100.00);
        grad.insert_hue(172.00, 220.00);
        grad.insert_saturation(172.00, 60.00);
        grad.insert_brightness(172.00, 100.00);
        grad.insert_hue(215.00, 239.53);
        grad.insert_saturation(215.00, 50.20);
        grad.insert_brightness(215.00, 100.00);
        grad.insert_hue(258.00, 260.00);
        grad.insert_saturation(258.00, 60.00);
        grad.insert_brightness(258.00, 100.00);
        grad.insert_hue(301.00, 274.04);
        grad.insert_saturation(301.00, 69.80);
        grad.insert_brightness(301.00, 100.00);
        grad.insert_hue(344.00, 285.00);
        grad.insert_saturation(344.00, 80.00);
        grad.insert_brightness(344.00, 100.00);
        grad.insert_hue(387.00, 293.48);
        grad.insert_saturation(387.00, 90.20);
        grad.insert_brightness(387.00, 100.00);
        grad.insert_hue(430.00, -60.00);
        grad.insert_saturation(430.00, 100.00);
        grad.insert_brightness(430.00, 100.00);
        grad.rescale(min, max);
        grad
    }
    pub fn spring(min: f32, max: f32) -> HsvGradient {
        let mut grad = HsvGradient::new(0.0, 430.00);
        grad.insert_hue(0.00, -60.00);
        grad.insert_saturation(0.00, 100.00);
        grad.insert_brightness(0.00, 100.00);
        grad.insert_hue(43.00, -53.48);
        grad.insert_saturation(43.00, 90.20);
        grad.insert_brightness(43.00, 100.00);
        grad.insert_hue(86.00, -45.37);
        grad.insert_saturation(86.00, 80.39);
        grad.insert_brightness(86.00, 100.00);
        grad.insert_hue(129.00, -34.53);
        grad.insert_saturation(129.00, 70.20);
        grad.insert_brightness(129.00, 100.00);
        grad.insert_hue(172.00, -20.00);
        grad.insert_saturation(172.00, 60.00);
        grad.insert_brightness(172.00, 100.00);
        grad.insert_hue(215.00, -0.47);
        grad.insert_saturation(215.00, 50.20);
        grad.insert_brightness(215.00, 100.00);
        grad.insert_hue(258.00, 20.00);
        grad.insert_saturation(258.00, 60.00);
        grad.insert_brightness(258.00, 100.00);
        grad.insert_hue(301.00, 34.04);
        grad.insert_saturation(301.00, 69.80);
        grad.insert_brightness(301.00, 100.00);
        grad.insert_hue(344.00, 45.00);
        grad.insert_saturation(344.00, 80.00);
        grad.insert_brightness(344.00, 100.00);
        grad.insert_hue(387.00, 53.48);
        grad.insert_saturation(387.00, 90.20);
        grad.insert_brightness(387.00, 100.00);
        grad.insert_hue(430.00, 60.00);
        grad.insert_saturation(430.00, 100.00);
        grad.insert_brightness(430.00, 100.00);
        grad.rescale(min, max);
        grad
    }
    pub fn summer(min: f32, max: f32) -> HsvGradient {
        let mut grad = HsvGradient::new(0.0, 430.00);
        grad.insert_hue(0.00, 167.81);
        grad.insert_saturation(0.00, 100.00);
        grad.insert_brightness(0.00, 50.20);
        grad.insert_hue(43.00, 160.17);
        grad.insert_saturation(43.00, 82.14);
        grad.insert_brightness(43.00, 54.90);
        grad.insert_hue(86.00, 150.29);
        grad.insert_saturation(86.00, 67.32);
        grad.insert_brightness(86.00, 60.00);
        grad.insert_hue(129.00, 137.33);
        grad.insert_saturation(129.00, 54.22);
        grad.insert_brightness(129.00, 65.10);
        grad.insert_hue(172.00, 120.00);
        grad.insert_saturation(172.00, 43.02);
        grad.insert_brightness(172.00, 70.20);
        grad.insert_hue(215.00, 103.15);
        grad.insert_saturation(215.00, 46.60);
        grad.insert_brightness(215.00, 74.90);
        grad.insert_hue(258.00, 90.00);
        grad.insert_saturation(258.00, 50.00);
        grad.insert_brightness(258.00, 80.00);
        grad.insert_hue(301.00, 80.35);
        grad.insert_saturation(301.00, 53.00);
        grad.insert_brightness(301.00, 85.10);
        grad.insert_hue(344.00, 72.19);
        grad.insert_saturation(344.00, 55.65);
        grad.insert_brightness(344.00, 90.20);
        grad.insert_hue(387.00, 65.53);
        grad.insert_saturation(387.00, 58.02);
        grad.insert_brightness(387.00, 95.29);
        grad.insert_hue(430.00, 60.00);
        grad.insert_saturation(430.00, 60.00);
        grad.insert_brightness(430.00, 100.00);
        grad.rescale(min, max);
        grad
    }
    pub fn autumn(min: f32, max: f32) -> HsvGradient {
        let mut grad = HsvGradient::new(0.0, 430.00);
        grad.insert_hue(0.00, 0.00);
        grad.insert_saturation(0.00, 100.00);
        grad.insert_brightness(0.00, 100.00);
        grad.insert_hue(43.00, 5.88);
        grad.insert_saturation(43.00, 100.00);
        grad.insert_brightness(43.00, 100.00);
        grad.insert_hue(86.00, 11.76);
        grad.insert_saturation(86.00, 100.00);
        grad.insert_brightness(86.00, 100.00);
        grad.insert_hue(129.00, 17.88);
        grad.insert_saturation(129.00, 100.00);
        grad.insert_brightness(129.00, 100.00);
        grad.insert_hue(172.00, 24.00);
        grad.insert_saturation(172.00, 100.00);
        grad.insert_brightness(172.00, 100.00);
        grad.insert_hue(215.00, 29.88);
        grad.insert_saturation(215.00, 100.00);
        grad.insert_brightness(215.00, 100.00);
        grad.insert_hue(258.00, 36.00);
        grad.insert_saturation(258.00, 100.00);
        grad.insert_brightness(258.00, 100.00);
        grad.insert_hue(301.00, 41.88);
        grad.insert_saturation(301.00, 100.00);
        grad.insert_brightness(301.00, 100.00);
        grad.insert_hue(344.00, 48.00);
        grad.insert_saturation(344.00, 100.00);
        grad.insert_brightness(344.00, 100.00);
        grad.insert_hue(387.00, 54.12);
        grad.insert_saturation(387.00, 100.00);
        grad.insert_brightness(387.00, 100.00);
        grad.insert_hue(430.00, 60.00);
        grad.insert_saturation(430.00, 100.00);
        grad.insert_brightness(430.00, 100.00);
        grad.rescale(min, max);
        grad
    }
    pub fn winter(min: f32, max: f32) -> HsvGradient {
        let mut grad = HsvGradient::new(0.0, 430.00);
        grad.insert_hue(0.00, 240.00);
        grad.insert_saturation(0.00, 100.00);
        grad.insert_brightness(0.00, 100.00);
        grad.insert_hue(43.00, 233.83);
        grad.insert_saturation(43.00, 100.00);
        grad.insert_brightness(43.00, 95.29);
        grad.insert_hue(86.00, 226.96);
        grad.insert_saturation(86.00, 100.00);
        grad.insert_brightness(86.00, 90.20);
        grad.insert_hue(129.00, 218.99);
        grad.insert_saturation(129.00, 100.00);
        grad.insert_brightness(129.00, 85.10);
        grad.insert_hue(172.00, 210.00);
        grad.insert_saturation(172.00, 100.00);
        grad.insert_brightness(172.00, 80.00);
        grad.insert_hue(215.00, 200.31);
        grad.insert_saturation(215.00, 100.00);
        grad.insert_brightness(215.00, 75.29);
        grad.insert_hue(258.00, 188.72);
        grad.insert_saturation(258.00, 100.00);
        grad.insert_brightness(258.00, 70.20);
        grad.insert_hue(301.00, 175.96);
        grad.insert_saturation(301.00, 100.00);
        grad.insert_brightness(301.00, 69.80);
        grad.insert_hue(344.00, 165.00);
        grad.insert_saturation(344.00, 100.00);
        grad.insert_brightness(344.00, 80.00);
        grad.insert_hue(387.00, 156.52);
        grad.insert_saturation(387.00, 100.00);
        grad.insert_brightness(387.00, 90.20);
        grad.insert_hue(430.00, 150.12);
        grad.insert_saturation(430.00, 100.00);
        grad.insert_brightness(430.00, 100.00);
        grad.rescale(min, max);
        grad
    }
}

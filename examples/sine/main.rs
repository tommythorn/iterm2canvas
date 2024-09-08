use iterm2canvas::*;

fn main() {
    let (width, height) = (1600, 800);
    let mut pict = Pict::new(width, height);

    // Sine
    for x in 0..width {
        let xf = x as f64 / width as f64;
        let yf = (xf * 2.0 * 3.1415926535).sin() + 1.0;
        assert!(0.0 <= yf && yf <= 2.0);
        let yf = yf / 2.0 * (height - 5) as f64 + 0.5;
        assert!(
            0.0 <= yf && yf < height as f64,
            "{yf} is not in [0; {height})"
        );
        let y = yf as isize;
        pict.plot(x, y, 0xFFFF00);
    }

    pict.dump_iterm2_image(None);
}

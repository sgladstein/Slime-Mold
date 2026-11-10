use std::time::Instant;
fn hash(mut x: u64) -> u64 { x ^= x >> 33; x = x.wrapping_mul(0xff51afd7ed558ccd); x ^= x >> 33; x = x.wrapping_mul(0xc4ceb9fe1a85ec53); x ^ (x >> 33) }
fn run(w: usize, h: usize, n: usize, steps: usize) {
    let mut trail = vec![0f32; w*h]; let mut scent = vec![0.1f32; w*h]; let mut tmp = vec![0f32; w*h];
    let mut wx: Vec<f32> = (0..n).map(|i| (hash(i as u64) % (w as u64 - 40)) as f32 + 20.0).collect();
    let mut wy: Vec<f32> = (0..n).map(|i| (hash(i as u64 + 7777) % (h as u64 - 40)) as f32 + 20.0).collect();
    let mut hd: Vec<f32> = (0..n).map(|i| (hash(i as u64 + 99) % 6283) as f32 / 1000.0).collect();
    let (sa, ra, so) = (0.785f32, 0.785f32, 9.0f32 * (w as f32 / 256.0));
    let mut tm = 0.0; let mut td = 0.0;
    for s in 0..steps {
        let t0 = Instant::now();
        for k in 0..n {
            let (x, y, h0) = (wx[k], wy[k], hd[k]);
            let sense = |a: f32| { let sx = (x + a.cos()*so) as isize; let sy = (y + a.sin()*so) as isize;
                if sx < 1 || sy < 1 || sx >= w as isize - 1 || sy >= h as isize -1 { -1e6 } else { let i = sy as usize * w + sx as usize; trail[i] + scent[i] } };
            let f = sense(h0); let fl = sense(h0 - sa); let fr = sense(h0 + sa);
            let mut hh = h0;
            if f > fl && f > fr {} else if f < fl && f < fr { hh += if hash((k as u64) << 20 ^ s as u64) & 1 == 0 { ra } else { -ra } } else if fl < fr { hh += ra } else if fr < fl { hh -= ra }
            let (nx, ny) = (x + hh.cos(), y + hh.sin());
            if nx > 1.0 && ny > 1.0 && nx < (w-2) as f32 && ny < (h-2) as f32 { wx[k]=nx; wy[k]=ny; hd[k]=hh; trail[ny as usize * w + nx as usize] += 5.0; } else { hd[k] = (hash(k as u64 ^ s as u64) % 6283) as f32 / 1000.0; }
        }
        let t1 = Instant::now();
        for f in [&mut trail, &mut scent] {
            for y in 1..h-1 { for x in 1..w-1 { let i = y*w+x; let mut a = 0.0;
                for dy in [-(w as isize), 0, w as isize] { for dx in [-1isize,0,1] { a += f[(i as isize + dy + dx) as usize]; } }
                tmp[i] = a / 9.0 * 0.9; } }
            std::mem::swap(f, &mut tmp);
        }
        let t2 = Instant::now();
        tm += (t1-t0).as_secs_f64(); td += (t2-t1).as_secs_f64();
    }
    let sum: f32 = trail.iter().sum();
    println!("{w}x{h} n={n}: move {:.2} ms/step, diffuse(2 fields) {:.2} ms/step, single core (chk {sum:.0})", tm*1e3/steps as f64, td*1e3/steps as f64);
}
fn main() { run(256,160,2200,200); run(1024,640,100_000,40); run(1024,640,1_000_000,10); run(2048,1280,1_000_000,10); }

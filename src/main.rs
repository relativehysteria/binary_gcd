use core::arch::x86_64::_rdtsc;
use core::ops::{ShrAssign, Shl};

fn rdtsc() -> u64 {
    unsafe { _rdtsc() }
}

fn naive(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        a %= b;
        core::mem::swap(&mut a, &mut b);
    }
    a
}

fn optimized(mut a: usize, mut b: usize) -> usize {
    if a == 0 { return b; }
    if b == 0 { return a; }

    let mut a_zeros = a.trailing_zeros();
    let b_zeros = b.trailing_zeros();
    let shift = a_zeros.min(b_zeros);
    b.shr_assign(b_zeros);

    while a != 0 {
        a.shr_assign(a_zeros);
        let diff = b - a;
        a_zeros = diff.trailing_zeros();
        b = b.min(a);
        a = diff;
    }

    return b.shl(shift);
}

fn main() {
    let a = rdtsc() as usize;
    let b = a / 2;

    let start = rdtsc();
    let nv = (0..1024 * 1024).reduce(|acc, _| acc + naive(a, b)).unwrap();
    let nv_cycles = rdtsc() - start;

    let start = rdtsc();
    let opt = (0..1024 * 1024).reduce(|acc, _| acc + optimized(a, b)).unwrap();
    let opt_cycles = rdtsc() - start;

    assert!(nv == opt);

    println!("naive cycles: {nv_cycles}");
    println!("optimized cycles: {opt_cycles}");
}

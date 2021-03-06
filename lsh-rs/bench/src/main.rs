#![feature(test)]
extern crate test;
use lsh_rs::{
    utils::rand_unit_vec, HashTables, LshSqlMem, MemoryTable, SignRandomProjections, SqlTable,
    SqlTableMem, LSH,
};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use test::Bencher;

fn prep_vecs(n: usize, dim: usize) -> Vec<Vec<f32>> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        let rng = SmallRng::seed_from_u64(i as u64);
        v.push(rand_unit_vec(dim, rng))
    }
    v
}

fn store_n(n: usize, dim: usize, index_only: bool) -> LSH<MemoryTable, SignRandomProjections> {
    let v = prep_vecs(n, dim);
    let mut lsh;
    if index_only {
        lsh = LSH::new(20, 7, 100).seed(1).only_index().srp().unwrap();
    } else {
        lsh = LSH::new(20, 7, 100).seed(1).srp().unwrap();
    }
    lsh.store_vecs(&v);
    lsh
}

#[bench]
fn bench_storing(b: &mut Bencher) {
    b.iter(|| store_n(100, 100, false))
}

#[bench]
fn bench_storing_index_only(b: &mut Bencher) {
    b.iter(|| store_n(100, 100, true))
}

#[bench]
fn bench_storing_sqlite_mem(b: &mut Bencher) {
    let mut lsh = LshSqlMem::new(20, 80, 100).seed(1).l2(4.).unwrap();
    b.iter(|| {
        let v = prep_vecs(100, 100);
        lsh.store_vecs(&v);
    })
}

#[bench]
fn bench_query(b: &mut Bencher) {
    let lsh = store_n(100, 100, false);

    let mut seed = 295;
    let rng = SmallRng::seed_from_u64(seed);
    let q = rand_unit_vec(100, rng);
    b.iter(|| {
        let rng = SmallRng::seed_from_u64(seed);
        let q = rand_unit_vec(100, rng);
        lsh.query_bucket(&q);
        seed += 1;
    });
}

#[bench]
fn bench_sqlite(b: &mut Bencher) {
    let mut sql = SqlTableMem::new(1, true, ".").unwrap();
    let v = vec![1., 2.];
    let hash = vec![1, 2];
    b.iter(|| {
        sql.put(hash.clone(), &v, 0);
    })
}

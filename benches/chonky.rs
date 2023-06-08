use smolldb::SmollDB;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion){
    c.bench_function("chonky db", |b| {
        let mut db = SmollDB::default();
        for i in 0..100 {
            db.set(format!("bool{}", i), true);
        }
        for i in 0..100 {
            db.set(format!("int8{}", i), i as i8);
        }
        for i in 0..100 {
            db.set(format!("int16{}", i), i as i16);
        }
        for i in 0..100 {
            db.set(format!("int32{}", i), i as i32);
        }
        for i in 0..100 {
            db.set(format!("int64{}", i), i as i64);
        }
        for i in 0..100 {
            db.set(format!("float32{}", i), i as f32);
        }
        for i in 0..100 {
            db.set(format!("float64{}", i), i as f64);
        }
        for i in 0..100 {
            db.set(format!("string{}", i), format!("string{}", i));
        }
        for i in 0..100 {
            db.set(format!("bytes{}", i), vec![i as u8; 100]);
        }
        b.iter(||{
            db.backup(black_box("chonky")).unwrap();
        });
        b.iter(|| {
            let db_copy = SmollDB::load(black_box("chonky")).unwrap();
            assert_eq!(db, db_copy);
        });
    });
    

}

criterion_group!(benches, benchmark);
criterion_main!(benches);
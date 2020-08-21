use init_trait::Init;

#[test]
fn init_empty_array() {
	let arr = <[usize; 0]>::init(|_| panic!("Shouldn't call init function"));
	assert_eq!(arr, []);
}

#[test]
fn init_singleton_array() {
	let mut called = false;
	let arr = <[usize; 1]>::init(|i| {
		assert_eq!(i, 0);
		if called { panic!("Should only call init function once"); }
		else { called = true; }
		123
	});
	assert!(called);
	assert_eq!(arr, [123]);
}

#[test]
fn init_array() {
	let arr = <[usize; 123]>::init(|i| i);
	for i in 0..123 {
		assert_eq!(arr[i], i);
	}
}

#[test]
fn init_2d_array() {
	let arr = <[[(usize, usize); 34]; 12] as Init<(usize, usize), [usize; 2]>>::init(|[x, y]| (x, y));
	for x in 0..12 {
		for y in 0..34 {
			assert_eq!(arr[x][y], (x, y));
		}
	}
}

#[test]
fn init_3d_array() {
	let arr = <[[[(usize, usize, usize); 34]; 23]; 12] as Init<(usize, usize, usize), [usize; 3]>>::init(|[x, y, z]| (x, y, z));
	for x in 0..12 {
		for y in 0..23 {
			for z in 0..34 {
				assert_eq!(arr[x][y][z], (x, y, z));
			}
		}
	}
}

#[test]
fn init_4d_array() {
	let arr = <[[[[(usize, usize, usize, usize); 5]; 4]; 3]; 2] as Init<(usize, usize, usize, usize), [usize; 4]>>::init(|[w, x, y, z]| (w, x, y, z));
	for w in 0..2 {
		for x in 0..3 {
			for y in 0..4 {
				for z in 0..5 {
					assert_eq!(arr[w][x][y][z], (w, x, y, z));
				}
			}
		}
	}
}

#[test]
fn init_5d_array() {
	let arr = <[[[[[(usize, usize, usize, usize, usize); 6]; 5]; 4]; 3]; 2] as Init<(usize, usize, usize, usize, usize), [usize; 5]>>::init(|[v, w, x, y, z]| (v, w, x, y, z));
	for v in 0..2 {
		for w in 0..3 {
			for x in 0..4 {
				for y in 0..5 {
					for z in 0..6 {
						assert_eq!(arr[v][w][x][y][z], (v, w, x, y, z));
					}
				}
			}
		}
	}
}

#[test]
fn init_6d_array() {
	let arr = <[[[[[[(usize, usize, usize, usize, usize, usize); 5]; 4]; 3]; 4]; 3]; 2] as Init<(usize, usize, usize, usize, usize, usize), [usize; 6]>>::init(|[u, v, w, x, y, z]| (u, v, w, x, y, z));
	for u in 0..2 {
		for v in 0..3 {
			for w in 0..4 {
				for x in 0..3 {
					for y in 0..4 {
						for z in 0..5 {
							assert_eq!(arr[u][v][w][x][y][z], (u, v, w, x, y, z));
						}
					}
				}
			}
		}
	}
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn init_empty_vec() {
	let arr = Vec::init_with(0, |_| panic!("Shouldn't call init function"));
	assert_eq!(arr, vec![]);
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn init_singleton_vec() {
	let mut called = false;
	let arr = Vec::init_with(1, |i| {
		assert_eq!(i, 0);
		if called { panic!("Should only call init function once"); }
		else { called = true; }
		123
	});
	assert!(called);
	assert_eq!(arr, vec![123]);
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn init_vec() {
	let arr = Vec::init_with(123, |i| i);
	assert_eq!(arr.len(), 123);
	for i in 0..123 {
		assert_eq!(arr[i], i);
	}
}

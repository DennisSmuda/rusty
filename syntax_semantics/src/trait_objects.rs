/**
 * Trait Objects
 */
let v = vec![1, 2, 3];
let o = &v as &Clone; // Will throw error - not object safe

// Static Traits can be turned into their own objects
// - object safe (No 'self')

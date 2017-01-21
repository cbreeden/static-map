Current benchmarks

```
test fx  ... bench:      16,638 ns/iter (+/- 753)
test phf ... bench:      78,374 ns/iter (+/- 4,220)
```

Fastest hash found for data:

```
Best: 472952511931313797
Avg: 0.22894795, 0 => 3504, 1 => 921, 2 => 52,
```

The default key is still faster for some reason, even though this key
has slightly fewer collisions.
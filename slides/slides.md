Optimisation des temps de calcul en Rust
========================================

Choix du language
=================

Avantages
---------

- Rapide
	- Bas niveau
	- Basé sur LLVM
- Sécurisé
	- Sans segmentation faults
	- Sûreté des threads
	- Overflow / Underflow

Inconvénients
-------------

- Sécurisé
	- Généricité vs. Templates
	- Pas de conversions implicites
- Support
	- Plateforme
	- Bibliothèques

Optimisations algorithmiques
============================

Médian
------

### Par tri

```rust
kernel.sort();
let median_color = kernel[5];
```

### O(n) / O(1)

```rust
let median_color = Luma([hist.median()]);
```

Sobel
-----

### Distance euclidienne

```rust
let root = (((gx * gx) + (gy * gy)) as f32).sqrt() as u8;
```

### Distance approchée

```rust
let root = (((gx.abs() + gy.abs()) >> 1) as f32 * 1.41421) as u8;
```

Optimisations "informatiques"
=============================

Optimisations de cache
----------------------

- Ordonnancement des itérations
- 😭 Dissection pour le multi-threading

Combinaisons
------------

- Combinaison du sobel et seuillage
```rust
    let root =
        if root > threshold {
            255
        } else {
            0
        };
```

- 😭 Combinaison de la conversion en niveau de gris et du filtre médian

Optimisations de compilations
=============================

Commandes
---------

### Lancement du projet

```sh
RUSTFLAGS="-C target-cpu=native" cargo run -- display
```

### Benchmarking

```sh
RUSTFLAGS="-C target-cpu=native" cargo bench
```

Merci de votre attention !
==========================



Just a simple raytracer in Rust using the CPU.

Showcase :

![screenshot](./images/showcase1High.png)


This scene rendering runtime took 40 minutes on my laptop.

![screenshot](./images/Rendered40min.png)




**Performance Benchmarks**

To evaluate the efficiency of different rendering pipelines, I compared the
execution times of several implementation strategies. All benchmarks were
executed using `cargo run --release`.

*   **Direct stdout stream:** 11.174s total (11.04s user)
*   **Buffered image-to-stdout:** 9.808s total (9.80s user)
*   **Binary P3 format implementation:** 9.503s total (9.48s user)

While the transition to binary output provided measurable performance gains,
the overall impact on total render time remains marginal.


**Parallelization with Rayon**

By integrating the `rayon` crate, I parallelized the rendering process to leverage multi-core CPU architectures. 

```rust
    {
        // Parallelization requires collecting pixels into a vector before 
        // reconstruction, as direct mutation is not thread-safe.
        let pixels: Vec<Vector3> = (0..self.height)
            .into_par_iter()
            .flat_map(|y| {
                (0..self.width).into_par_iter().map(move |x| {
                    let mut pixel_f = Vector3::new(0.0, 0.0, 0.0);

                    for _ in 0..self.samples_per_pixel {
                        let mut hit_record = HitRecord::new();
                        hit_record.t_min = EPSILON;
                        hit_record.t_max = INFINITY;

                        let mut recursion_depth = 0;

                        let ray = get_ray_at_coordinates(x, y, &self.camera);
                        pixel_f =
                            pixel_f + self.ray_color(ray, &mut hit_record, &mut recursion_depth);
                    }

                    pixel_f * (1.0 / self.samples_per_pixel as f32)
                })
            })
            .collect();

        Imagef32 {
            width: self.width,
            height: self.height,
            pixels,
        }
    }
```

**Performance Results**
*   **Execution Time:** 2.956s total (32.55s user, 1101% CPU utilization).
*   **Trade-off:** This approach significantly improves throughput but sacrifices real-time progress reporting in the console.

**Output Sample (Rendered in 1 minute 44 seconds)**

![screenshot](./images/parllelism1.png)


**Development and Optimization**

The Rust ecosystem significantly streamlined the development process. In particular, the `rayon` crate provided an ergonomic and efficient solution for implementing data parallelism with minimal overhead.

During development, a visual artifacting issue was identified, stemming from inconsistent state initialization within the `HitRecord` struct (a discrepancy between `.reset()` and `::new()`). This has since been resolved, ensuring rendering accuracy.

**Enhanced Monitoring and High-Resolution Rendering**

To facilitate longer, high-complexity renders, I integrated the `indicatif` crate to provide a thread-safe progress bar. This allows for real-time tracking of multi-threaded workloads:

`[00:00:04][######>---------------------------------] 174/1080 (25s)`

The following high-quality render was completed in **8 minutes**, utilizing all available CPU cores while maintaining system stability for background tasks.

![screenshot](./images/parallism2bigimage.png)

**Performance Impact: 40 Minutes vs. 8 Minutes**

The transition from a 40-minute sequential render to an 8-minute parallelized render represents a **5x increase in performance (an 80% reduction in wall-clock time)**. 

While the sequential implementation was bottlenecked by single-core throughput, the parallel implementation effectively saturates the CPU's computational capacity. This optimization shifts the project from a proof-of-concept into a functional tool capable of producing high-fidelity images in a fraction of the time.



**Memory Layout and Struct Optimization**

Analysis of the `HitRecord` struct revealed potential memory inefficiencies. In Rust, structs can incur significant overhead due to padding to satisfy alignment requirements. By utilizing the `#[repr(C)]` attribute, I ensured a predictable memory layout.

```rust
#[repr(C)]
pub struct HitRecord {
    pub point: Vector3,   // 12 bytes
    pub normal: Vector3,  // 12 bytes
    pub t: f32,           // 4 bytes
    pub t_min: f32,       // 4 bytes
    pub t_max: f32,       // 4 bytes
    pub front_face: bool, // 1 byte
    pub current_sphere: usize, // 4/8 bytes
}
```

While the immediate performance gain from packing was marginal, optimizing this structure is critical for scaling. Since `HitRecord` is frequently allocated and modified during the recursive ray-tracing process, reducing its memory footprint minimizes cache misses and allocator pressure.

**Acceleration Structures: Bounding Volume Hierarchy (BVH)**

To move beyond the linear complexity of checking every object for every ray, I implemented a **Bounding Volume Hierarchy (BVH)**. This algorithm partitions the scene into a tree of Axis-Aligned Bounding Boxes (AABB), reducing the intersection search complexity from $O(n)$ to $O(\log n)$.

The implementation dynamically chooses the longest axis to split the volumes, ensuring a balanced tree that maximizes pruning during the traversal of the `hit_bvh` function:

```rust
pub fn hit_bvh(node: &BVHNode, ray: &Ray, hit_record: &mut HitRecord, spheres: &Spheres) -> bool {
    // Early exit if the ray doesn't hit the node's bounding box
    if !node.bounding_box.is_hit(ray, hit_record.t_min, hit_record.t_max) {
        return false;
    }

    // Leaf node: perform actual sphere intersection
    if node.left.is_none() && node.right.is_none() {
        let sphere_index = node.sphere.index;
        let center = spheres.spheres_centers[sphere_index];
        let radius = spheres.spheres_radius[sphere_index];
        let hit = is_hit_sphere(*ray, center, radius, hit_record);
        if hit {
            hit_record.current_sphere = sphere_index;
        }
        return hit;
    }

    // Recursive traversal
    let hit_left = if let Some(left) = &node.left {
        hit_bvh(left, ray, hit_record, spheres)
    } else { false };

    let hit_right = if let Some(right) = &node.right {
        hit_bvh(right, ray, hit_record, spheres)
    } else { false };

    hit_left || hit_right
}
```

**BVH Performance Results**

The integration of the BVH provided a massive reduction in wall-clock time, even when compared to the previous parallelized version:

*   **Parallel Render (Linear):** 8 minutes
*   **Parallel Render (BVH Optimized):** 2 minutes
*   **Performance Gain:** 4x improvement over parallel baseline (20x faster than initial sequential render).

**Enhanced Visual Showcase**

Beyond raw speed, the BVH implementation allows for much denser scenes. The following render features a high concentration of small, reflective spheres. The increased sampling and complexity yield a significantly "crispier" image with complex light paths and reflections.

![screenshot](./images/BVHfirstImage.png)

Next : GPU

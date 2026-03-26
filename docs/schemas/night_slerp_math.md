# Night SLERP Math Pseudocode

## Purpose
Pseudocode for a fitness function involving Spherical Linear Interpolation (SLERP) in a "Night" context (e.g., nighttime optimization or simulation).

## Assumptions
- SLERP interpolates between two unit vectors/quaternions on a sphere.
- "Night" mode applies time-based weighting or constraints.
- Fitness function evaluates interpolation quality or convergence.

## Pseudocode
```
function slerp(v0, v1, t):
    dot = v0 · v1
    if dot > 0.9995:
        return normalize(v0 + t * (v1 - v0))  # Linear approx for small angles
    theta = acos(dot)
    sin_theta = sin(theta)
    a = sin((1 - t) * theta) / sin_theta
    b = sin(t * theta) / sin_theta
    return a * v0 + b * v1

function night_fitness(interpolations, target_time, night_factor):
    # night_factor: 0.0 (day) to 1.0 (night), affects weighting
    total_error = 0
    for each interp in interpolations:
        result = slerp(interp.start, interp.end, interp.t * night_factor)
        error = distance(result, interp.target)  # e.g., Euclidean or angular distance
        weighted_error = error * (1 + night_factor)  # Boost during night
        total_error += weighted_error
    fitness = 1 / (1 + total_error)  # Higher fitness for lower error
    return fitness
```

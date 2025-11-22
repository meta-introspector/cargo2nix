# Feature Lattice Build Results

This report summarizes the build characteristics (time and binary size) for various feature permutations of the `git-wrapper-lib` crate, as measured by the `feature-permutation-builder`.

## Summary of Results

| Feature Set                | Status  | Build Time (ms) | Binary Size (bytes) | Source Hash                              | Timestamp (UTC)     |
| :------------------------- | :------ | :-------------- | :------------------ | :--------------------------------------- | :------------------ |
| (no features)              | SUCCESS | 3014            | N/A                 | `abb553824cf9cfd2768d81c44e0cbab2b31b0f79` | 2025-11-22 12:19:37 |
| `git2`                     | SUCCESS | 3368            | N/A                 | `022b88901fdebf01fe580ab3aeef23f2a19f99bd` | 2025-11-22 12:19:40 |
| `with-anyhow`              | SUCCESS | 2094            | N/A                 | `74e549d75b99e12fb10906793c876ad55bb53423` | 2025-11-22 12:19:43 |
| `with-trace`               | SUCCESS | 3076            | N/A                 | `71fd09ef7f1c5f7f87a94e716b2e434daf55b1d3` | 2025-11-22 12:19:45 |
| `git2,with-anyhow`         | SUCCESS | 2259            | N/A                 | `712d4f34119649dcf2ac38cf162890674bc49fac` | 2025-11-22 12:19:49 |
| `git2,with-trace`          | SUCCESS | 3726            | N/A                 | `e860d66247e358210ca0d4d6c39e731226ebecd9` | 2025-11-22 12:19:51 |
| `with-anyhow,with-trace`   | SUCCESS | 2124            | N/A                 | `5b0377b4448887e86befe287789849695494f58d` | 2025-11-22 12:19:55 |
| `git2,with-anyhow,with-trace` | SUCCESS | 2396            | N/A                 | `ade2f457d71f688354e51411cc25c19c715dde3f` | 2025-11-22 12:19:57 |

## Observations

*   All tested feature combinations successfully built.
*   The `git2` feature generally adds to the build time, likely due to its C library dependencies.
*   `with-anyhow` and `with-trace` features have a relatively small impact on build times individually and in combination.
*   Binary size information is currently not captured, indicated by "N/A". This could be improved in future iterations of the `feature-permutation-builder`.

## Future Work

*   Implement binary size extraction in `feature-permutation-builder`.
*   Expand the range of feature permutations tested.
*   Integrate this report generation into a CI/CD pipeline for continuous monitoring.

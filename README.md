# libmapper-rs

Rust wrapper for the libmapper C library. Allows creation and manipulation of libmapper signals and devices.

## Compatibility

Below is a table of the libmapper-rs versions and the libmapper versions they are compatible with.

| libmapper-rs | libmapper |
|--------------|-----------|
| 1.0.0-1.1.0  | 2.4.7     |
| 1.1.0-1.1.1  | 2.4.9     |

## Notes
- Libmapper 2.4.9 has a bug causing the pointer to object IDs to be unaligned. This causes rust to panic when calling `get_property` to get the ID.
Turning on release optimizations will mitigate this as it skips rust's alignment check.
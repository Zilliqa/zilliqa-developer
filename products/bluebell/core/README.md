# Bluebell core

This crate contains the base infrastructure of the Scilla compiler, Bluebell.

## Useful references

An example of source location can he found
[here](https://github.com/gluon-lang/gluon/blob/f8326d21a14b5f21d203e9c43fa5bb7f0688a74c/base/src/source.rs#L22-L35)

Memory formatter in Python:

```python
def f(x):
	print('\n'.join([str(hex(i*32)).ljust(4) + ": " + x[64*i:64*(i+1)] for i in range(int(len(x)/64))]))
```

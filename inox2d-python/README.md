# Inox2D Python Bindings

Python bindings for [Inox2D](https://github.com/Inochi2D/inox2d), an experimental Rust port of the [Inochi2D](https://github.com/Inochi2D/inochi2d) puppet animation specification.

## Installation

```sh
pip install inox2d
```

## Usage

```python
import inox2d

# Create a context
ctx = inox2d.Context()

# Load a model
ctx.load_model("path/to/model.inp")
```

## Status

**Currently this library and the specification is in a prototype state**, it is not recommended to use this library in production.

These bindings currently expose basic model loading capabilities. More features from the core library will be exposed in the future.

## License

This project is licensed under the 2-Clause BSD license.
See [LICENSE](https://github.com/Inochi2D/inox2d/blob/main/LICENSE) for details.

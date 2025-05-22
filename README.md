Python Tecplot PLT Reader
========================

This project provides Python utilities to parse Tecplot binary (`.plt`) files.
It uses the [`construct`](https://pypi.org/project/construct/) library to
interpret the binary structures and [`numpy`](https://numpy.org/) for handling
arrays of values.

Requirements
------------
* construct
* numpy

Example
-------
```python
import tecplotPltReader

# Load the PLT file
with open('data.plt', 'rb') as f:
    raw = f.read()
    header = tecplotPltReader.read_header(raw)
    data = tecplotPltReader.read_data(raw, header, f)

print(header['Title'])
print(data['Zones'][0].keys())
```


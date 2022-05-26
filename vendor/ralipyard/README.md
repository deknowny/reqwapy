# Ralipyard
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/ralipyard)
![PyPI - Implementation](https://img.shields.io/pypi/implementation/ralipyard)
![PyPI](https://img.shields.io/pypi/v/ralipyard)

Python thread-safe rate limit semaphore for threading and async/await style built on Rust's library [raliguard](https://github.com/deknowny/ralipyard)

### Overview
For example, crate a semaphore that allows only 5 calls per 1 second
```python
import asyncio
import time
import datetime

import ralipyard


async def task(sem: ralipyard.Semaphore, no: int):
    delay = sem.calc_delay()
    await asyncio.sleep(delay)
    print(f"Task {no=} executes its code at {time.monotonic():.5f}")


async def main():
    sem = ralipyard.Semaphore(
        access_times=5,
        per_period=datetime.timedelta(seconds=1)
    )
    tasks = [task(sem, i) for i in range(100)]
    await asyncio.gather(*tasks)


asyncio.run(main())
```
Output:
```shell
Task no=0 executes its code at 0.21575
Task no=4 executes its code at 1.22018
Task no=3 executes its code at 1.22022
Task no=2 executes its code at 1.22023
Task no=1 executes its code at 1.22024
Task no=9 executes its code at 2.21618
Task no=7 executes its code at 2.21622
Task no=8 executes its code at 2.21623
Task no=6 executes its code at 2.21624
Task no=5 executes its code at 2.21624
Task no=11 executes its code at 3.22016
Task no=10 executes its code at 3.22020
Task no=12 executes its code at 3.22021
Task no=14 executes its code at 3.22022
Task no=13 executes its code at 3.22023
...
```

### Instalation
From PyPI:
```shell
python -m pip install ralipyard
```
Or from GitHub
```shell
python -m pip install https://github.com/deknowny/archive/ralipyard.zip
```

### Documentation
It's unnecessary because the library has the only 1 class with the only 2 methods, visit [stubs](./ralipyard.pyi) for more information

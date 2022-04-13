# ftd-py

A PyPI package to use FTD.

## Usage

1. `fpm_build`: This can be used to build the fpm package

```python
from ftd import fpm_build
import asyncio

async def build(file=None, base_url=None, ignore_failed=None):
    await fpm_build(file, base_url, ignore_failed)

loop = asyncio.get_event_loop()
loop.run_until_complete(build())
loop.close()
```

2. `render`: This can be used to build a single ftd file and also inject the 
   variable data for processor of type `get-data`

```python
import json
import ftd
import asyncio


async def render():
   data = {"message": "hello world", "n": 10}
   data = json.dumps(data)
   await examples.render("foo.ftd", data)


loop = asyncio.get_event_loop()
loop.run_until_complete(render())
loop.close()
```

The above code can be used to render the `foo.ftd` file defined below

```ftd
/-- This is foo.ftd file

-- string message:
$processor$: get-data

-- integer n:
$processor$: get-data

-- ftd.text: $message

-- ftd.integer: $n
```

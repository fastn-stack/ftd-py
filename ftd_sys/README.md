# ftd-py

A PyPI package to use FTD.

## Usage

1. `fpm_build`: This can be used to build the fpm package

```python
from ftd_sys import fpm_build
import asyncio

async def build(root=None, file=None, base_url=None, ignore_failed=None):
    await fpm_build(root, file, base_url, ignore_failed)

loop = asyncio.get_event_loop()
loop.run_until_complete(build())
loop.close()
```

2. `render`: This can be used to build a single ftd file and also inject the 
   variable data for processor of type `get-data`

```python
import json
import ftd_sys
import asyncio


async def render(root=None):
   data = {"message": "hello world", "n": 10}
   data = json.dumps(data)
   await ftd_sys.render(root, "foo", data)


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

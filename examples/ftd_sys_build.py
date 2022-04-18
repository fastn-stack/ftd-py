from ftd_sys import fpm_build
import asyncio


async def build(root=None, file=None, base_url=None, ignore_failed=None):
    await fpm_build(root, file, base_url, ignore_failed)


asyncio.run(build(root="../sample"))

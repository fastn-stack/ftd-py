from ftd_sys import fpm_build
import asyncio
import os


def ftd_build(path, file=None, base_url=None, ignore_failed=None):
    async def build():
        await fpm_build(file, base_url, ignore_failed)

    current_dir = os.getcwd()
    os.chdir(path)
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    loop.run_until_complete(build())
    loop.close()
    os.chdir(current_dir)

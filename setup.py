from setuptools import setup

with open("README.md") as f:
    long_description = f.read()

setup(
    author="Amit Upadhyay",
    author_email="upadhyay@gmail.com",
    description="A PyPI package to use FTD",
    long_description=long_description,
    long_description_content_type="text/markdown",
    name="ftd",
    url="https://github.com/FifthTry/ftd-py",
    project_urls={
        "Bug Tracker": "https://github.com/FifthTry/ftd-py/issues",
    },
)

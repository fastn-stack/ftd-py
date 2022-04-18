from setuptools import setup

with open("README.md") as f:
    long_description = f.read()

setup(
    author="Amit Upadhyay",
    author_email="upadhyay@gmail.com",
    description="Helper for using `ftd` as template language in your django project.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    name="ftd-django",
    url="https://github.com/FifthTry/ftd-py",
    project_urls={
        "Bug Tracker": "https://github.com/FifthTry/ftd-py/issues",
    },
    python_requires=">=3.6",
    version="0.1.0",
    install_requires=["ftd"],
)

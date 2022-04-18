from setuptools import setup, find_packages

with open("README.md") as f:
    long_description = f.read()

setup(
    author="Amit Upadhyay",
    author_email="upadhyay@gmail.com",
    description="Python Binding For FTD",
    long_description=long_description,
    long_description_content_type="text/markdown",
    name="ftd",
    url="https://github.com/FifthTry/ftd-py",
    project_urls={
        "Bug Tracker": "https://github.com/FifthTry/ftd-py/issues",
    },
    python_requires=">=3.6",
    version="0.1.2",
    install_requires=["ftd_sys>=0.1.0"],
    package_dir={"": "src"},
    packages=find_packages(where="src"),
)

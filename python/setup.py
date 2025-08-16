from setuptools import setup, find_packages

setup(
    name="draviavemal_openxml_office",
    version="4.0.0",
    packages=find_packages(),
    include_package_data=True,
    ext_modules=[],
    zip_safe=False,
    setup_requires=["cffi"],
    install_requires=["cffi"],
    cffi_modules=["build_bindings.py:ffi"],
)

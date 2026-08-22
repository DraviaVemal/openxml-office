# OpenXML Office (Python Wrapper)

[![PyPI](https://img.shields.io/pypi/v/draviavemal_openxml_office.svg)](https://pypi.org/project/draviavemal_openxml_office/)

## Short Description
OpenXML Office is a Python wrapper built around the Rust-based core implementation of OpenXML Office. It facilitates creating, manipulating, and managing OpenXML documents such as **Word documents (.docx)**, **Excel spreadsheets (.xlsx)**, and **PowerPoint presentations (.pptx)**, while leveraging the high performance of the Rust core through a C FFI layer using `cffi`.

## Mission
My mission is to provide a seamless and efficient Python interface for OpenXML document processing by utilizing the Rust core. This wrapper is designed to:

- Bridge the performance benefits of Rust with the ease of use of Python.
- Support cross-platform development with Python 3.8 and above.
- Ensure feature parity with the Rust-based core package.

## Package Details

- **PyPI Package:** [draviavemal_openxml_office on PyPI](https://pypi.org/project/draviavemal_openxml_office/)
- **Minimum Supported Version:** Python 3.8

> **Note:** This wrapper relies on the C FFI layer of the Rust-based core package via `cffi`. The Rust CFFI and Python wrapper documentation are ongoing activities and may not yet fully reflect the implemented features. Refer to the main project documentation and source for the latest functionality.

## Working Samples

To see sample code and working tests, please refer to the test files:
[GitHub: Alpha Test Files](https://github.com/DraviaVemal/openxml-office/tree/release/tests/python)

These tests demonstrate various use cases and can serve as a starting point for integrating the wrapper into your Python projects.

## Additional Information

This Python wrapper is located in the `python` directory and serves as a Python interface for the OpenXML Office project. It supports:

- **Python 3.8** (minimum)

### Platform Support

- **Windows:** Supported
- **Linux:** Supported
- **Mac:** Support in progress

For more information on the overarching project, visit the main repository:
[GitHub: OpenXML Office Main Repository](https://github.com/DraviaVemal/openxml-office)

## License

This project is dual-licensed. See the [LICENSE](https://github.com/DraviaVemal/openxml-office/blob/release/LICENSE) file for details.

## Contributing

Contributions are welcome! Whether it’s reporting bugs, suggesting improvements, or submitting pull requests, your help is greatly appreciated. For more details, see our [CONTRIBUTING](https://github.com/DraviaVemal/openxml-office/blob/release/CONTRIBUTING.md) guidelines.

---

For inquiries, feedback, or contributions, feel free to message me via GitHub or submit an issue. Thank you for supporting the OpenXML Office project!
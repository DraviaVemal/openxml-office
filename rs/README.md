# OpenXML Office (Rust Edition)

[![Crates.io](https://img.shields.io/crates/v/draviavemal-openxml_office.svg)](https://crates.io/crates/draviavemal-openxml_office)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://openxml-office.draviavemal.com/v4.x-alpha)

## Short Description
OpenXML Office is a Rust-based package designed for creating, manipulating, and managing OpenXML documents such as **Word documents (.docx)**, **Excel spreadsheets (.xlsx)**, and **PowerPoint presentations (.pptx)**. This package eliminates the dependency on OpenXML-SDK, offering a blazing-fast, standalone solution for handling these document formats.

## Mission
My mission is to provide a highly efficient and feature-rich Rust library for OpenXML document processing. With this rewrite, I aim to:

- Increase performance and reduce dependency overhead.
- Deliver an intuitive and extensible API for document creation and manipulation.
- Ensure compatibility with the latest OpenXML standards.

## Package Details

- **Crate:** [draviavemal-openxml_office on crates.io](https://crates.io/crates/draviavemal-openxml_office)
- **Documentation:** [v4.x-alpha Documentation](https://openxml-office.draviavemal.com/v4.x-alpha)

> **Note:** Documentation is an ongoing activity and may not yet fully reflect the implemented features. Refer to the source and tests for the latest functionality.

## Working Samples

To see sample code and working tests, please refer to the test files:
[GitHub: Alpha Test Files](https://github.com/DraviaVemal/openxml-office/tree/alpha/rs/src/tests)

These tests demonstrate various use cases and can serve as a starting point for integrating the library into your projects.

## Additional Information

This Rust package serves as the core implementation for the OpenXML Office project. It is located in the `rs` directory and is designed to support other programming languages through CFFI integration. For more information on the overarching project, visit the main repository:
[GitHub: OpenXML Office Main Repository](https://github.com/DraviaVemal/openxml-office)

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/DraviaVemal/openxml-office/blob/main/LICENSE) file for details.

## Contributing

Contributions are welcome! Whether it’s reporting bugs, suggesting improvements, or submitting pull requests, your help is greatly appreciated. For more details, see our [CONTRIBUTING](https://github.com/DraviaVemal/openxml-office/blob/main/CONTRIBUTING.md) guidelines.

---

For inquiries, feedback, or contributions, feel free to message me via GitHub or submit an issue. Thank you for supporting the OpenXML Office project!


## TODO

[ ] DashMap optimization for parallel construct, update and deconstruct xml
[ ] Add space to pass custom parsing in current XML deserializer to improve part specific case like SheetData
[ ] Improve Style Options and lookups
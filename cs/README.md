# OpenXML Office (C# Wrapper)

[![NuGet](https://img.shields.io/nuget/v/draviavemal.openxml-office.svg)](https://www.nuget.org/packages/draviavemal.openxml-office)

## Short Description
OpenXML Office is a C# wrapper built around the Rust-based core implementation of OpenXML Office. It facilitates creating, manipulating, and managing OpenXML documents such as **Word documents (.docx)**, **Excel spreadsheets (.xlsx)**, and **PowerPoint presentations (.pptx)**, while leveraging the high performance of the Rust core through a C FFI layer.

## Mission
My mission is to provide a seamless and efficient .NET interface for OpenXML document processing by utilizing the Rust core. This wrapper is designed to:

- Bridge the performance benefits of Rust with the ease of use of .NET.
- Support cross-platform development with .NET 6.0 and .NET Standard 2.1 compatibility.
- Ensure feature parity with the Rust-based core package.

## Package Details

- **NuGet Package:** [draviavemal.openxml-office on NuGet](https://www.nuget.org/packages/draviavemal.openxml-office)

> **Note:** This wrapper relies on the C FFI layer of the Rust-based core package. The Rust CFFI and C# wrapper documentation are ongoing activities and may not yet fully reflect the implemented features. Refer to the main project documentation and source for the latest functionality.

## Working Samples

To see sample code and working tests, please refer to the test files:
[GitHub: Alpha Test Files](https://github.com/DraviaVemal/openxml-office/tree/alpha/cs/test)

These tests demonstrate various use cases and can serve as a starting point for integrating the wrapper into your .NET projects.

## Additional Information

This C# wrapper is located in the `cs` directory and serves as a .NET interface for the OpenXML Office project. It supports:

- **.NET 6.0** (minimum)
- **.NET Standard 2.1** (minimum)

### Platform Support

- **Windows:** Supported
- **Linux:** Supported
- **Mac:** Support in progress

For more information on the overarching project, visit the main repository:
[GitHub: OpenXML Office Main Repository](https://github.com/DraviaVemal/openxml-office)

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/DraviaVemal/openxml-office/blob/main/LICENSE) file for details.

## Contributing

Contributions are welcome! Whether it’s reporting bugs, suggesting improvements, or submitting pull requests, your help is greatly appreciated. For more details, see our [CONTRIBUTING](https://github.com/DraviaVemal/openxml-office/blob/main/CONTRIBUTING.md) guidelines.

---

For inquiries, feedback, or contributions, feel free to message me via GitHub or submit an issue. Thank you for supporting the OpenXML Office project!


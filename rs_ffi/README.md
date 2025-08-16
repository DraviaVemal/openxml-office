# OpenXML Office FFI (Rust)

[![Crates.io](https://img.shields.io/crates/v/draviavemal-openxml_office_ffi.svg)](https://crates.io/crates/draviavemal-openxml_office_ffi)

## Short Description

`draviavemal-openxml_office_ffi` is a Rust crate that serves as the bridge between the core implementation of the `draviavemal-openxml_office_ffi` library written in Rust and multiple programming languages. By exposing the core functionality of OpenXML Office via a C FFI interface, this crate allows developers to manipulate OpenXML documents (Word, Excel, and PowerPoint) from languages such as C#, Python, Go, TypeScript, Java, and any other language that supports C FFI.

## Mission

This crate aims to provide a high-performance, flexible C FFI interface to the OpenXML Office Rust library, ensuring that:

- **Cross-language support** is enabled, simplifying the integration with C#, Python, Go, TypeScript, Java, and more.
- **Performance** is maximized, leveraging the speed and efficiency of Rust for document manipulation.
- **Extensibility** allows developers to expand support for additional languages or systems as needed.

## Package Details

- **Crate Name**: `draviavemal-openxml_office_ffi`
- **Crate Documentation**: [Docs.rs: draviavemal-openxml_office_ffi](https://docs.rs/draviavemal-openxml_office_ffi)
- **Repository**: [GitHub: OpenXML Office](https://github.com/DraviaVemal/openxml-office/tree/alpha/rs_ffi)

The `rs_ffi` directory of the OpenXML Office repository contains the Rust FFI crate and serves as the foundational layer for exposing the core OpenXML Office functionality to various languages.

## Usage

This crate exposes a C FFI interface that can be used by developers in different programming languages (such as C#, Python, Go, TypeScript, Java) to interact with OpenXML documents. 

For language-specific integration guides and documentation, refer to the respective directories for each language implementation (e.g., `cs`, `python`, `go`).

## Platform Support

- **Windows**: Fully supported
- **Linux**: Fully supported
- **MacOS**: Support under development

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/DraviaVemal/openxml-office/blob/main/LICENSE) file for details.

## Contributing

Contributions are welcome! You can help by reporting issues, suggesting improvements, or submitting pull requests. Please follow the [CONTRIBUTING](https://github.com/DraviaVemal/openxml-office/blob/main/CONTRIBUTING.md) guidelines.

---

For inquiries, feedback, or contributions, feel free to message via GitHub or submit an issue. Thank you for supporting the `openxml-office-ffi` project!

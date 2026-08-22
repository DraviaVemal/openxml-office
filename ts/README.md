# OpenXML Office (TypeScript Wrapper)

[![npm](https://img.shields.io/npm/v/draviavemal-openxml_office.svg)](https://www.npmjs.com/package/draviavemal-openxml_office)

> **Status:** Phase 2 — planned. This wrapper is under active development and may not yet be published.

## Short Description
OpenXML Office is a TypeScript/Node.js wrapper built around the Rust-based core implementation of OpenXML Office. It facilitates creating, manipulating, and managing OpenXML documents such as **Word documents (.docx)**, **Excel spreadsheets (.xlsx)**, and **PowerPoint presentations (.pptx)**, while leveraging the high performance of the Rust core exposed as a native Node addon through **NAPI-RS**.

## Mission
My mission is to provide a seamless and efficient TypeScript interface for OpenXML document processing by utilizing the Rust core. This wrapper is designed to:

- Bridge the performance benefits of Rust with the ease of use of TypeScript/Node.js.
- Expose the core library as a native Node addon using NAPI-RS.
- Ensure feature parity with the Rust-based core package.

## Package Details

- **npm Package:** [draviavemal-openxml_office on npm](https://www.npmjs.com/package/draviavemal-openxml_office)
- **Binding:** NAPI-RS is used to expose the core lib as a Node addon.

> **Note:** This wrapper relies on the NAPI-RS binding of the Rust-based core package. The Rust binding and TypeScript wrapper documentation are ongoing activities and may not yet fully reflect the implemented features. Refer to the main project documentation and source for the latest functionality.

## Working Samples

To see sample code and working tests, please refer to the test files:
[GitHub: Alpha Test Files](https://github.com/DraviaVemal/openxml-office/tree/alpha/ts)

These tests demonstrate various use cases and can serve as a starting point for integrating the wrapper into your TypeScript/Node.js projects.

## Additional Information

This TypeScript wrapper is located in the `ts` directory and serves as a Node.js interface for the OpenXML Office project.

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

# OpenXML Office (Rust API Server)

[![Docker Hub](https://img.shields.io/docker/v/draviavemal/openxml-office?label=Docker%20Hub)](https://hub.docker.com/r/draviavemal/openxml-office)

## Short Description
OpenXML Office API is a containerized HTTP service built on top of the Rust-based core implementation of OpenXML Office. It exposes the core capabilities for creating, manipulating, and managing OpenXML documents such as **Word documents (.docx)**, **Excel spreadsheets (.xlsx)**, and **PowerPoint presentations (.pptx)** over the network, with support for HTTP and QUIC.

## Mission
My mission is to provide a high-performance, language-agnostic API for OpenXML document processing by running the Rust core as a service. This API is designed to:

- Expose the Rust core over HTTP with QUIC support.
- Enable integration from any language or platform through a simple network interface.
- Ensure feature parity with the Rust-based core package.

## Package Details

- **Crate:** `draviavemal-openxmloffice_rs_api`
- **Container Image:** [OpenXML Office API on Docker Hub](https://hub.docker.com/r/draviavemal/openxml-office)
- **Framework:** Built with `actix-web`.

> **Note:** This API server wraps the Rust-based core package. Documentation is an ongoing activity and may not yet fully reflect the implemented features. Refer to the main project documentation and source for the latest functionality.

## Working Samples

To see sample code and working tests, please refer to the source and test files:
[GitHub: Alpha Source Files](https://github.com/DraviaVemal/openxml-office/tree/alpha/rs_api)

These serve as a starting point for integrating the API container into your infrastructure.

## Additional Information

This API server is located in the `rs_api` directory and serves as the HTTP/QUIC interface for the OpenXML Office project.

### Platform Support

- **Windows:** Supported
- **Linux:** Supported
- **Mac:** Support in progress

For more information on the overarching project, visit the main repository:
[GitHub: OpenXML Office Main Repository](https://github.com/DraviaVemal/openxml-office)

## License

This project is dual-licensed. See the [LICENSE](https://github.com/DraviaVemal/openxml-office/blob/main/LICENSE) file for details.

## Contributing

Contributions are welcome! Whether it’s reporting bugs, suggesting improvements, or submitting pull requests, your help is greatly appreciated. For more details, see our [CONTRIBUTING](https://github.com/DraviaVemal/openxml-office/blob/main/CONTRIBUTING.md) guidelines.

---

For inquiries, feedback, or contributions, feel free to message me via GitHub or submit an issue. Thank you for supporting the OpenXML Office project!

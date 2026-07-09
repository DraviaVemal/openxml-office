# draviavemal-openxml_office (AGPL-3.0)

## Effortless Creation of Excel, PowerPoint, and Word Documents

This project aims to provide a streamlined, efficient way to create and manipulate Excel, PowerPoint, and Word documents using the OpenXML format. By leveraging the power of modern programming languages, this library simplifies the process of document creation, enabling developers to focus on functionality rather than the intricacies of the OpenXML standard.

### 📜 License

This project is **dual-licensed**:

- **Open Source — AGPL-3.0** (see [LICENSE](LICENSE)). Free to use, including
  commercially, as long as you comply with AGPL-3.0 copyleft (share source of
  derivatives and network-served versions).
- **Commercial License** for closed-source / proprietary use without AGPL
  obligations — available to sponsors (see [COMMERCIAL-LICENSE.md](COMMERCIAL-LICENSE.md)).

💜 If this project helps you or your business, please consider
[sponsoring](https://github.com/sponsors/DraviaVemal) — sponsors get the
commercial license, private releases, and priority support.

## v2.x - Stable Version (C# Only) Open-XML-SDK based

Please refer [stable branch](https://github.com/DraviaVemal/OpenXML-Office/tree/stable) for V2/Stable Package related details. This is active development Branch

## Stable Status
| Detail            | Status                                                                                                                                                                                                                                       | Detail                | Status                                                                                                                                                                                                                                            |
| ----------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Stable Release    | [![GitHub Release](https://img.shields.io/github/v/release/DraviaVemal/OpenXML-Office?sort=semver&label=Stable%20Release)](https://openxml-office.draviavemal.com/)                                                                          | Stable Build Status   | [![Package Build and Publish to NuGet](https://github.com/DraviaVemal/openxmloffice/actions/workflows/nuget-publish-stable.yml/badge.svg?branch=stable)](https://github.com/DraviaVemal/openxmloffice/actions/workflows/nuget-publish-stable.yml) |
| Alpha Release     | [![NuGet](https://img.shields.io/nuget/vpre/openxmloffice.Presentation.svg)](https://www.nuget.org/packages/openxmloffice.Presentation)                                                                                                      | Alpha Build Status    | [![Package Build and Publish to NuGet](https://github.com/DraviaVemal/openxmloffice/actions/workflows/nuget-publish-alpha.yml/badge.svg?branch=alpha)](https://github.com/DraviaVemal/openxmloffice/actions/workflows/nuget-publish-alpha.yml)    |
| Code Quality      | [![Codacy Badge](https://app.codacy.com/project/badge/Grade/5b420a599805426ab8a990a1a741247a)](https://app.codacy.com/gh/DraviaVemal/OpenXML-Office/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)       | PPT Files Generated   | [![Generated](https://draviavemal.com/openxml-office/powerpoint-count.svg)](https://openxml-office.draviavemal.com/)                                                                                                                              |
| Code Coverage     | [![Codacy Badge](https://app.codacy.com/project/badge/Coverage/5b420a599805426ab8a990a1a741247a)](https://app.codacy.com/gh/DraviaVemal/OpenXML-Office/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_coverage) | Excel Files Generated | [![Generated](https://draviavemal.com/openxml-office/excel-count.svg)](https://openxml-office.draviavemal.com/)                                                                                                                                   |
| Package Downloads | [![Downloads](https://img.shields.io/nuget/dt/openxmloffice.Presentation.svg)](https://www.nuget.org/packages/openxmloffice.Presentation)                                                                                                    | Word Files Generated  | [![Generated](https://draviavemal.com/openxml-office/word-count.svg)](https://openxml-office.draviavemal.com/)                                                                                                                                    |

## Important Note about v3.x (Discontinued) 🚫

After thorough analysis, I have concluded that the full OpenXML format relations and connections are adequately addressed. Therefore, to eliminate duplicate work, V3 is no longer under development. Please follow V4 updates for final results.

# v4.x Release and Targets 🚧🛠️

Docs - [Will Try to keep the alpha release documents updated as much possible](https://openxml-office.draviavemal.com/v4.x-alpha)

| Supported Languages | Min.Support Version | Readme link                                                                         | Packages                   | package link                                                       | Description                                                               |
| ------------------- | ------------------- | ----------------------------------------------------------------------------------- | -------------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| Rust                | 1.32                | [README](https://github.com/DraviaVemal/openxml-office/blob/alpha/rs/README.md)     | draviavemal-openxml_office | [Crates](https://crates.io/crates/draviavemal-openxml_office)      | Rust crate directly connecting to core lib                                |
| C#                  | .net6.0             | [README](https://github.com/DraviaVemal/openxml-office/blob/alpha/cs/README.md)     | draviavemal.openxml-office | [Nuget](https://www.nuget.org/packages/draviavemal.openxml-office) | C# wrapper package wrote around FFI layer of rust                         |
| Python              | 3.8                 | [README](https://github.com/DraviaVemal/openxml-office/blob/alpha/python/README.md) | Python                     | [PyPi](https://pypi.org/)                                          | Python Wrapper package wrote around FFI layer of rust using cffi          |
| ----------------    | ----------          | ---------                                                                           | --------                   | ---------                                                          | -------                                                                   |
| PHASE 2             | PHASE 2             | PHASE 2                                                                             | PHASE 2                    | PHASE 2                                                            | PHASE 2                                                                   |
| ---------------     | -----------         | ---------                                                                           | --------                   | ----------                                                         | -------                                                                   |
| Java                | 1.8                 | TODO                                                                                | Java                       | [Maven Central](https://mvnrepository.com/)                        | Java wrapper package wrote around FFI layer of rust                       |
| Go                  | 1.22                | TODO                                                                                | Go                         | [Github](https://github.com/DraviaVemal/OpenXML-Office/)           | Go wrapper package wrote around FFI layer of rust                         |
| TypeScript          |                     | TODO                                                                                | TypeScript                 | [npm](https://www.npmjs.com/)                                      | NAPI-RS is used to expose the core lib as node addon                      |
|                     |                     |                                                                                     | Rust-API                   | [Docker Hub](https://hub.docker.com/)                              | API container running rust crate for HTTP support along with quic support |

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated. If you have suggestions that could improve this project, please fork the repo and create a pull request. Alternatively, you can open an issue tagged "enhancement." Don’t forget to star the project!

### How to Contribute

1. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
2. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
3. Push to the Branch (`git push origin feature/AmazingFeature`)
4. Open a Pull Request 

Please ensure you follow the PR and issue templates for quicker resolution.

## Support

Your feedback and support are important. Feel free to reach out with any questions or suggestions.

+++
template = "landing.html"

[extra.hero]
title = "Welcome to Smugglex"
badge = "v0.1.0"
description = "Powerful open-source HTTP Request Smuggling testing tool written in Rust, designed for security testing and penetration testing to detect various types of HTTP request smuggling vulnerabilities."
image = "/images/preview.jpg" # Background image
cta_buttons = [
    { text = "Get Started", url = "/get_started/installation", style = "primary" },
    { text = "View on GitHub", url = "https://github.com/hahwul/smugglex", style = "secondary" },
]

[extra.features_section]
title = "Essential Features"
description = "Discover Smugglex's essential features for comprehensive HTTP request smuggling vulnerability detection and analysis."

[[extra.features]]
title = "Multiple Attack Vectors"
desc = "Test for CL.TE, TE.CL, TE.TE, H2C, and H2 smuggling attack vectors with extensive payload variations."
icon = "fa-solid fa-network-wired"

[[extra.features]]
title = "Timing-Based Detection"
desc = "Advanced timing-based detection algorithms to identify desynchronization vulnerabilities in web applications and proxies."
icon = "fa-solid fa-clock"

[[extra.features]]
title = "Flexible Input Modes"
desc = "Support for command-line URLs, stdin pipeline, and various scanning modes for integration with other security tools."
icon = "fa-solid fa-terminal"

[[extra.features]]
title = "Payload Export"
desc = "Export vulnerable payloads for further analysis and exploitation in authorized testing scenarios."
icon = "fa-solid fa-download"

[[extra.features]]
title = "High Performance"
desc = "Built with Rust and async operations for efficient concurrent scanning with progress tracking and timeout handling."
icon = "fa-solid fa-bolt"

[[extra.features]]
title = "Comprehensive Coverage"
desc = "Detect Reflected, Stored, and protocol-level smuggling with support for HTTP/1.1 and HTTP/2 protocols."
icon = "fa-solid fa-shield-halved"

[extra.final_cta_section]
title = "Contributing"
description = "Smugglex is an open-source project made with ❤️. If you want to contribute to this project, please see CONTRIBUTING.md and submit a pull request with your cool content!"
button = { text = "View Contributing Guide", url = "https://github.com/hahwul/smugglex/blob/main/CONTRIBUTING.md" }
+++

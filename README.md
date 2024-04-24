![typenotion GitHub Banner](https://github.com/subtalegames/typenotion/assets/24438483/f15eba5c-af22-4f0f-82ef-2a7600e7a99b)

[![OSS by Subtale](https://img.shields.io/badge/oss_by-subtale-f0f0f1?style=flat-square&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyBpZD0iTGF5ZXJfMiIgZGF0YS1uYW1lPSJMYXllciAyIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI3MTQuOTciIGhlaWdodD0iNDYxLjEzIiB2aWV3Qm94PSIwIDAgNzE0Ljk3IDQ2MS4xMyI%2BCiAgPGcgaWQ9IkxheWVyXzEtMiIgZGF0YS1uYW1lPSJMYXllciAxIj4KICAgIDxnPgogICAgICA8cGF0aCBkPSJNMzU3LjQ4LDM1Ny41MWgtMjIyLjc4Yy05Ljk1LDAtMTkuNS0zLjk1LTI2LjU0LTEwLjk5TDAsMjM4LjM1aDIzOC4zMmwxMTkuMTYsMTE5LjE2WiIgZmlsbD0iI2ZmZiIgc3Ryb2tlLXdpZHRoPSIwIi8%2BCiAgICAgIDxwYXRoIGQ9Ik03MTQuOTcsMjM4LjM1bC0yMTEuNzgsMjExLjc4Yy0xNC42NiwxNC42Ni0zOC40MiwxNC42Ni01My4wOCwwbC05Mi42Mi05Mi42MiwxMTkuMTYtMTE5LjE2aDIzOC4zMloiIGZpbGw9IiNmZmYiIHN0cm9rZS13aWR0aD0iMCIvPgogICAgICA8cGF0aCBkPSJNNDc2LjY3LDExMC43M3YxNi45MWMtMjguMzEsMC01Ni42OCwxMC44Mi03OC4zMiwzMi40Ny0yMS42NSwyMS41OC0zMi40MSw0OS44OS0zMi40MSw3OC4yNmgtMTYuOTFjMC0yOC4zMS0xMC44Mi01Ni42OC0zMi40Ny03OC4zMi0yMS41OC0yMS42NS00OS44OS0zMi40LTc4LjI2LTMyLjR2LTE2LjkxYzI4LjMxLDAsNTYuNjgtMTAuODIsNzguMzItMzIuNDcsMjEuNjUtMjEuNTgsMzIuNDEtNDkuODksMzIuNDEtNzguMjZoMTYuOTFjMCwyOC4zMSwxMC44Miw1Ni42OCwzMi40Nyw3OC4zMiwyMS41OCwyMS42NSw0OS44OSwzMi40LDc4LjI2LDMyLjRaIiBmaWxsPSIjZmZmIiBzdHJva2Utd2lkdGg9IjAiLz4KICAgIDwvZz4KICA8L2c%2BCjwvc3ZnPg%3D%3D&logoColor=f0f0f1&labelColor=2060d3)][oss]
[![Chat on Discord](https://img.shields.io/badge/chat_on-discord-f0f0f1?style=flat-square&logo=discord&logoColor=f0f0f1&labelColor=2060d3)][discord]
[![Rust](https://img.shields.io/github/actions/workflow/status/subtalegames/typenotion/rust.yml?style=flat-square&color=f0f0f1&logo=github-actions&logoColor=f0f0f1&labelColor=2060d3)][gh-workflow]
[![MIT License](https://img.shields.io/badge/license-MIT-f0f0f1?style=flat-square&labelColor=2060d3)][mit]
[![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-f0f0f1?style=flat-square&labelColor=2060d3)][apache]

> `typenotion` is a CLI tool for generating Rust types from data stored in Notion sites.
>
> *Not affiliated with Notion!*

## Example

For example, given the following Notion database:

![Notion example screenshot](https://github.com/subtalegames/typenotion/assets/24438483/51d357cb-69c2-48b5-9a56-bf5b60839dbc)

Running the command:

```shell
typenotion enum <DATABASE_ID> --generate-docs --generate-display --derive=Copy --derive=Clone
```

will generate a file named `player_variants.rs` with the following source code:

```rs
// Generated at 2024-04-08T22:36:31.574003+00:00

#[derive(Copy, Clone)]
enum PlayerVariants {
    /// ## Player with Full Upgrades
    ///
    /// This is the variant applied to the player when they’re fully upgraded.
    ///
    /// This variant overrides all other variants (i.e. even if they have a weapon equipped, this variant will take precedence).
    ///
    /// [Link to Notion record](https://www.notion.so/Player-with-Full-Upgrades-0123...)
    PlayerWithFullUpgrades,
    /// ## Player with Tool
    ///
    /// This is the variant applied to the player when they’re wielding a tool.
    ///
    /// [Link to Notion record](https://www.notion.so/Player-with-Tool-0123...)
    PlayerWithTool,
    /// ## Default Player
    ///
    /// This is the default player variant (considered “base” or “standard”). They have no tools, weapons, or upgrades.
    ///
    /// [Link to Notion record](https://www.notion.so/Default-Player-0123...)
    DefaultPlayer,
    /// ## Player with Weapon
    ///
    /// This is the variant applied to the player when they’re currently holding a weapon.
    ///
    /// [Link to Notion record](https://www.notion.so/Player-with-Weapon-0123...)
    PlayerWithWeapon,
}

impl std::fmt::Display for PlayerVariants {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PlayerVariants::*;
        match self {
            PlayerWithFullUpgrades => write!(f, "Player with Full Upgrades"),
            PlayerWithTool => write!(f, "Player with Tool"),
            DefaultPlayer => write!(f, "Default Player"),
            PlayerWithWeapon => write!(f, "Player with Weapon"),
        }
    }
}
```

## License

typenotion is free and open source. Unless explicitly noted otherwise, all code in this repository is dual-licensed under the [MIT License][mit] and [Apache License, Version 2.0][apache].

This licensing approach is the de facto standard within the Rust ecosystem.

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

![Jurassic Park GIF](https://c.tenor.com/6IxQzcFGAkMAAAAC/tenor.gif)

[oss]: https://subtale.dev
[discord]: https://discord.subtale.com
[gh-workflow]: https://github.com/subtalegames/typenotion/actions/workflows/rust.yml
[mit]: LICENSE-MIT
[apache]: LICENSE-APACHE

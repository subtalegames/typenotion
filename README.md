# typenotion

[![OSS by Subtale](https://img.shields.io/badge/oss_by-subtale-white?style=flat-square&labelColor=14213D&color=E5E5E5)][oss]
[![Rust](https://img.shields.io/github/actions/workflow/status/subtalegames/notion-enum-codegen/rust.yml?style=flat-square&labelColor=14213D&color=E5E5E5)][gh-workflow]
[![MIT License](https://img.shields.io/badge/license-MIT-brightgreen?style=flat-square&labelColor=14213D&color=E5E5E5)][mit]
[![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-brightgreen?style=flat-square&labelColor=14213D&color=E5E5E5)][apache]

> `typenotion` is a CLI tool for generating Rust types from data stored in Notion sites.
>
> *Not affiliated with Notion!*

## Example

For example, given the following Notion database:

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

[oss]: https://oss.subtale.com
[gh-workflow]: https://github.com/subtalegames/mimir/actions/workflows/rust.yml
[mit]: LICENSE-MIT
[apache]: LICENSE-APACHE

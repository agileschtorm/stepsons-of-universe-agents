# rifrl Remote Inspection

Inspection date: March 28, 2026

Repository:

- Repo: `stepsons-of-universe/rifrl`
- Visibility: private
- Default branch: `main`
- URL: `https://github.com/stepsons-of-universe/rifrl`
- Latest observed `main` commit: `de9495dd3e3455c19eeb962829d61f0989c8a709`
- Latest observed `main` commit date: March 15, 2026 02:00:31 UTC
- Latest observed `main` commit message: `update clone-depends.sh`

Top-level repository contents observed remotely:

- `.gitignore`
- `.lldbinit`
- `3rdPartyLicenses/`
- `Cargo.toml`
- `Makefile`
- `build_all.bat`
- `clone-depends.sh`
- `controls.md`
- `data/`
- `dev-plan.txt`
- `doc/`
- `large-scale-plan.md`
- `pull_all.bat`
- `requirements.txt`
- `resources/`
- `src/`

Primary language mix reported by GitHub:

- Rust: 282275 bytes
- Makefile: 3801 bytes
- Python: 3589 bytes
- Shell: 2002 bytes
- Lua: 1570 bytes
- Batchfile: 737 bytes

Rust manifest summary:

- Package name: `stepsonsrl`
- Package version: `0.2.3`
- Rust edition: `2021`
- UI and rendering stack includes `macroquad`, `egui-macroquad`, `egui_extras`, and `image`
- ECS and roguelike stack includes `legion` and `bracket-lib`
- Lua scripting is enabled through `mlua` with `lua54`, `vendored`, `macros`, `serialize`, and `anyhow`

Local path dependencies declared in `Cargo.toml`:

- `../depends/step-talk`
- `../depends/step-buffs`
- `../depends/stepsons-shadowcasting`
- `../depends/rlua_json`
- `../depends/lua_ecs`
- `../depends/macroquad-tiled-redux`
- `../depends/thin-walls`
- `../depends/thin-walls-visibility`
- `../depends/step-combat`
- `../depends/step-ui`
- `../depends/step-percentage`
- `../depends/step-engine`
- `../depends/step-ecs`

Source tree observed at `src/`:

- `src/animation/`
- `src/assets.rs`
- `src/camera.rs`
- `src/color.rs`
- `src/components.rs`
- `src/effects/`
- `src/game/`
- `src/item/`
- `src/main.rs`
- `src/map/`
- `src/mob/`
- `src/platform/`
- `src/point.rs`
- `src/spawner.rs`
- `src/systems/`
- `src/talk/`
- `src/time.rs`
- `src/turn_state/`
- `src/ui/`

Documentation tree observed at `doc/`:

- `doc/images/`
- `doc/npc/`
- `doc/scripting/`
- `doc/talk-cookbook.md`
- `doc/talk-howto.md`
- `doc/talk_lua_manual.md`
- `doc/tiled-maps.md`

Workspace implications:

- `rifrl` expects a multi-repo local workspace, not a standalone checkout.
- `clone-depends.sh` clones repositories into `../depends/`.
- The requested flat `/root/step/<repo>` layout is not directly compatible with the current `rifrl` dependency layout.
- Before first clone, choose one of these approaches:
  - Maintain a compatibility `depends/` directory.
  - Create symlinks from `depends/<repo>` to flat repo directories.
  - Patch local bootstrap/build scripts after cloning.

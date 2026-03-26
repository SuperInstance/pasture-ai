# SuperInstance to PastureAI Rename Audit

This is an exhaustive list of all occurrences of "SuperInstance" in the repository, based on a full grep scan (excluding target directory). The list is grouped by category as per the task: code (Cargo.toml, src/*), docs (READMEs, announce, tutorials), metadata (.github), tests (none found explicitly), and other relevant files.

No changes are made; this is purely an audit for renames needed.

## Code (Cargo.toml, src/*)

- **Cargo.toml**
  - Line 2: name = "superinstance"
  - Line 5: authors = ["SuperInstance Ranch <ranch@superinstance.ai>"]
  - Line 11: documentation = "https://docs.rs/superinstance"
  - Line 13: homepage = "https://superinstance.ai"
  - Line 14: repository = "https://github.com/SuperInstance/superinstance"
  - Line 51: name = "superinstance"
  - Line 55: name = "superinstance"
  - Line 60: name = "superinstance-onboard"

- **Cargo.lock**
  - Line 3099: name = "superinstance" (Note: Generated file, may regenerate after rename)

- **src/main.rs**
  - Line 1: //! SuperInstance - The AI Ranch Ecosystem
  - Line 58: /// SuperInstance - The AI Ranch Ecosystem
  - Line 308: info!("  🌙 Run `superinstance` to start your own Ranch");
  - Line 324: info!("  ║     🐄 SUPERINSTANCE RANCH - The AI Ecosystem 🐄        ║ ");

- **src/web/mod.rs**
  - Line 1: //! SuperInstance Web Dashboard - Axum + Dioxus
  - Line 30: //! SuperInstance requires: ONE Rust binary.

- **src/web/api.rs**
  - Line 1: //! REST API endpoints for the SuperInstance Ranch

- **src/web/dashboard.rs**
  - Line 44: <title>SuperInstance Ranch</title>
  - Line 51: <h1>🐄 SuperInstance Ranch</h1>
  - Line 303: // SuperInstance Dashboard - Minimal reactive layer

- **src/channels/discord.rs**
  - Line 3: //! Provides a Discord bot interface for SuperInstance using the Serenity library.
  - Line 263: /// Create a SuperInstance status embed
  - Line 271: .title("🐄 SuperInstance Ranch Status")
  - Line 278: .footer("SuperInstance v0.1.0")

- **src/channels/webhook.rs**
  - Line 3: //! Provides a generic webhook interface for SuperInstance.

- **src/channels/telegram.rs**
  - Line 3: //! Provides a Telegram bot interface for SuperInstance.

- **src/lib.rs**
  - Line 1: //! # SuperInstance - The AI Ranch Ecosystem
  - Line 5: //! SuperInstance is not a monolithic AI, but a ranch of specialized intelligences
  - Line 46: //! use superinstance::{Ranch, Config};
  - Line 56: //!         superinstance::species::Intent::new("reason", "Analyze this code")

- **src/onboarding/mod.rs**
  - Line 361: summary.push_str("# SuperInstance Configuration Summary\n\n");

- **src/onboarding/wizard.rs**
  - Line 3: //! Provides an interactive command-line wizard for setting up SuperInstance.
  - Line 27: println!("\n{} SuperInstance is already configured!", CHECK);
  - Line 33: println!("{}", style("║        SUPERINSTANCE - The Ranch Ecosystem                ║").cyan());
  - Line 70: println!("This wizard will help you set up your SuperInstance Ranch.");
  - Line 147: println!("Connect your SuperInstance to communication channels:");
  - Line 419: println!("  {} Run 'superinstance' to start the Ranch", style("1.").dim());

- **src/onboarding/config.rs**
  - Line 7: /// Full SuperInstance configuration
  - Line 9: pub struct SuperInstanceConfig {
  - Line 24: impl Default for SuperInstanceConfig {
  - Line 55: name: "SuperInstance Ranch".to_string(),
  - Line 164: impl SuperInstanceConfig {
  - Line 230: let config = SuperInstanceConfig::default();

- **src/dashboard.rs**
  - Line 154: Span::styled("SUPERINSTANCE RANCH", Style::default()

- **src/bin/onboard.rs**
  - Line 1: //! SuperInstance Onboarding CLI
  - Line 3: //! Run this to set up your SuperInstance Ranch for the first time.
  - Line 16: #[command(name = "superinstance-onboard")]
  - Line 17: #[command(about = "SuperInstance Onboarding Wizard", long_about = None)]

- **src/pasture/inference.rs**
  - Line 7: //! This removes the hardware gate and lets anyone try SuperInstance.

- **src/species/duck.rs**
  - Line 61: user_agent: "SuperInstance-Duck/1.0".to_string(),

## Docs (READMEs, announce, tutorials)

- **src/README.md**
  - Line 24: │                          SUPERINSTANCE RUNTIME                               │
  - Line 310: │  Rust (SuperInstance)                                                       │

- **docs/tutorials/01-the-arrival.md**
  - Line 69: 5. Create user: `rancher` / `superinstance`
  - Line 95: ## Step 4: Install SuperInstance (5 minutes)
  - Line 99: curl -sSL https://install.superinstance.ai | bash
  - Line 102: git clone https://github.com/SuperInstance/superinstance.git
  - Line 103: cd superinstance
  - Line 118: ║        SUPERINSTANCE RANCH - Day 1                                    ║
  - Line 156: - ✅ Installed SuperInstance

- **docs/tutorials/02-the-morning-routine.md**
  - Line 14: The Morning Routine is SuperInstance's signature demo - a fully automated morning briefing that runs locally in under 5 seconds with zero cloud calls.
  - Line 229: │  SERVICE              CLOUD COST      SUPERINSTANCE                        │

- **docs/tutorials/00-quick-start.md**
  - Line 20: curl -sSL https://install.superinstance.ai | bash
  - Line 33: cd superinstance
  - Line 41: ║        SUPERINSTANCE RANCH - Day 1                        ║
  - Line 91: Your SuperInstance is now:

- **README.md** (root)
  - Multiple lines with "SuperInstance" in titles, badges, descriptions, etc. (e.g., lines 1,3,4,5,8,9,14,44,94,111,244,344,355,359,362,459,460,507,608,706,744,777-779,786,859,867,881)

- **manifesto.md**
  - Line 1: # THE SUPERINSTANCE MANIFESTO
  - Line 7: ## We are building a SuperInstance.
  - Line 9: A SuperInstance is a **Ranch**. It is an ecosystem of specialized intelligences (Livestock) managed by a loyal Foreman (Border Collie). It starts capable but generic, and becomes **superuseful** through evolution, breeding, and culling.
  - Line 112: **Build the SuperInstance. Make it useful. Make it evolve.**

- **ROADMAP-v1.0.md**
  - Line 1: # SuperInstance v1.0 Production Roadmap

- **IMPLEMENTATION.md**
  - Line 1: # SuperInstance v1.0 - Phase 1-2 Implementation
  - Line 198: │              SuperInstance Ranch                 │

- **NOTIFICATION.md**
  - Line 1: # OpenClaw Notification - SuperInstance v1.0 Production COMPLETE
  - Line 6: SuperInstance is now ready for production deployment!
  - Line 41: - **Release**: https://github.com/SuperInstance/superinstance/releases/tag/v1.0.0
  - Line 51: - GitHub Release: https://github.com/SuperInstance/superinstance/releases/tag/v1.0.0
  - Line 58: curl -sSL https://install.superinstance.ai | bash
  - Line 60: git clone https://github.com/SuperInstance/superinstance.git
  - Line 61: cd superinstance && make install
  - Line 77: **Release**: https://github.com/SuperInstance/superinstance/releases/tag/v1.0.0

- **pasture/README.md**
  - Line 20: The Pasture is where your agents live. Unlike traditional AI systems that store configuration in opaque databases, SuperInstance uses the filesystem itself as the source of truth. This means:
  - Line 295: cargo run --release --bin superinstance-onboard

- **pasture/cattle/hello-cow-v1/breed.md**
  - Line 32: You are a friendly greeter for the SuperInstance Ranch.

- **pasture/cattle-v1/breed.toml**
  - Line 39: You handle the heavy lifting of LLM inference on the SuperInstance Ranch.

## Metadata (.github)

- **.github/workflows/ci.yml**
  - Line 1: name: SuperInstance CI
  - Line 41: working-directory: superinstance
  - Line 45: working-directory: superinstance
  - Line 49: working-directory: superinstance
  - Line 53: working-directory: superinstance
  - Line 71: working-directory: superinstance
  - Line 75: working-directory: superinstance
  - Line 88: test -f superinstance/README.md
  - Line 89: test -f superinstance/src/README.md
  - Line 90: test -f superinstance/pasture/README.md
  - Line 91: test -f superinstance/genetics/README.md
  - Line 94: run: test -f superinstance/.env.example
  - Line 97: run: test ! -f superinstance/.env
  - Line 115: working-directory: superinstance
  - Line 134: working-directory: superinstance
  - Line 138: tar -czvf superinstance-${{ github.ref_name }}-linux-x86_64.tar.gz \
  - Line 139: -C superinstance/target/release superinstance \
  - Line 140: -C superinstance README.md Makefile \
  - Line 141: -C superinstance pasture \
  - Line 142: -C superinstance genetics
  - Line 147: files: superinstance-*.tar.gz

## Tests
No explicit test files mentioned in the grep output, but if there are any in src/tests/*, they may need scanning. None found in this scan.

## Other Files

- **scripts/install_jetson.sh** (multiple lines with "SuperInstance", "superinstance")
- **scripts/install.sh** (multiple lines)
- **.env.example** Line 1: # SuperInstance Environment Configuration
- **Makefile** (multiple lines with "SuperInstance", "superinstance")
- **night_school/breed.py** Lines 3 and 379
- **.gitignore** Line 98: # SUPERINSTANCE SPECIFIC

This list covers all matches from the grep scan. For renames, consider case sensitivity, URLs, and generated files.
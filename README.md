# ESP32-C6 Agentic Firmware Development

![ESP32-C6](https://img.shields.io/badge/ESP32--C6-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## 🚀 Overview

This repository demonstrates **agentic template-based firmware development** for the ESP32-C6 microcontroller using Rust and Claude Code. It's designed to showcase LLM-driven development workflows with comprehensive logging, test-driven development, and reusable patterns.

### Key Features
- ✅ **Progressive Tutorial Structure** - Learn ESP32-C6 Rust development step-by-step
- ✅ **Claude Code Integration** - Slash commands, custom workflows, and AI-assisted development
- ✅ **Template-Based Patterns** - Reusable code patterns for common embedded tasks
- ✅ **Comprehensive Logging** - Debug and verify firmware behavior through extensive logging
- ✅ **Test-Driven Development** - Tests for embedded Rust (unit, integration, HIL)
- ✅ **Modern Rust Ecosystem** - Latest Rust embedded libraries and best practices

## 📚 Tutorial Structure

Each lesson is a self-contained project with its own documentation, build configuration, and learning objectives.

### Lessons

#### Foundations
- **[01-blinky](./lessons/01-blinky/)** - GPIO output, basic firmware structure, logging
- **02-button-input** *(Coming Soon)* - GPIO input, interrupts, debouncing
- **03-state-machine** *(Coming Soon)* - Finite state machines, traffic light example

#### Peripherals
- **04-i2c-sensor** *(Coming Soon)* - I2C protocol, sensor drivers (BME280)
- **05-spi-display** *(Coming Soon)* - SPI communication, display control
- **06-uart-shell** *(Coming Soon)* - Serial communication, CLI interface

#### Advanced Topics
- **07-async-tasks** *(Coming Soon)* - Embassy async runtime, concurrent tasks
- **08-wifi-basics** *(Coming Soon)* - WiFi connectivity, network protocols
- **09-ota-updates** *(Coming Soon)* - Over-the-air firmware updates
- **10-sensor-fusion** *(Coming Soon)* - Multiple sensors, data processing

## 🛠️ Getting Started

### Prerequisites

1. **Install Rust and ESP Toolchain**
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Install ESP Rust toolchain
   cargo install espup
   espup install
   source $HOME/export-esp.sh
   ```

2. **Install Flashing Tools**
   ```bash
   cargo install espflash
   cargo install cargo-espflash
   ```

3. **Clone This Repository**
   ```bash
   git clone https://github.com/shanemmattner/esp32-c6-agentic-firmware.git
   cd esp32-c6-agentic-firmware
   ```

### Quick Start

1. Navigate to a lesson:
   ```bash
   cd lessons/01-blinky
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Flash to your ESP32-C6:
   ```bash
   cargo run --release
   # Or specify port manually:
   espflash flash target/riscv32imc-esp-espidf/release/blinky --port /dev/cu.usbserial-10
   ```

4. Monitor serial output:
   ```bash
   espflash monitor /dev/cu.usbserial-10
   ```

## 🤖 Claude Code Integration

This repository is designed to demonstrate best practices for LLM-driven embedded development.

### Slash Commands *(Coming Soon)*
- `/generate-driver` - Generate I2C/SPI/UART driver templates
- `/add-logging` - Automatically add comprehensive logging to code
- `/create-test` - Generate test scaffolding for embedded tests
- `/prd` - Create Product Requirements Document
- `/erd` - Generate Entity Relationship Diagrams

### Claude Code Configuration
The `.claude/` directory contains:
- **Project context** - System architecture, coding standards
- **Custom commands** - Firmware-specific development workflows
- **Templates** - Reusable code patterns for common tasks

## 📝 Development Workflow

### 1. Planning Phase
- Create PRD (Product Requirements Document)
- Design ERD (Entity Relationship Diagram) if applicable
- Define acceptance criteria

### 2. Test-Driven Development
- Write tests first (unit, integration, hardware-in-loop)
- Implement features to pass tests
- Verify with comprehensive logging

### 3. Implementation
- Use template patterns from previous lessons
- Add extensive logging at key points
- Document decisions and tradeoffs

### 4. Verification
- Run tests (unit, integration, on-device)
- Monitor serial logs for expected behavior
- Iterate based on feedback

## 🔬 Testing Strategy

### Unit Tests
Run on host machine, mock hardware:
```bash
cargo test
```

### Integration Tests
Run on actual ESP32-C6 hardware:
```bash
cargo test --features integration
```

### Hardware-in-Loop (HIL)
Automated tests with connected sensors/peripherals:
```bash
cargo test --features hil
```

## 📖 Documentation

Each lesson includes:
- **README.md** - Overview, learning objectives, instructions
- **PRD.md** *(where applicable)* - Requirements and specifications
- **Code comments** - Inline explanations and reasoning
- **Logging output** - Expected serial monitor messages

## 🎯 Target Audience

- **Rust Beginners** interested in embedded systems
- **Embedded Developers** exploring Rust
- **AI/LLM Enthusiasts** learning prompt engineering for code
- **Educators** teaching embedded Rust development

## 🤝 Contributing

Contributions are welcome! This project is designed for learning and experimentation.

## 📄 License

This project is licensed under the MIT License - see LICENSE file for details.

## 🙏 Acknowledgments

- [esp-rs](https://github.com/esp-rs) - Rust on ESP community
- [Espressif](https://www.espressif.com/) - ESP32-C6 hardware
- [Anthropic](https://www.anthropic.com/) - Claude Code AI assistant

## 📺 YouTube Series *(Coming Soon)*

This repository will be featured in an upcoming YouTube series on:
- Agentic firmware development
- LLM-driven embedded development
- Modern Rust embedded patterns
- ESP32-C6 development

## 🔗 Resources

- [ESP32-C6 Datasheet](https://www.espressif.com/sites/default/files/documentation/esp32-c6_datasheet_en.pdf)
- [esp-idf-svc Documentation](https://docs.esp-rs.org/esp-idf-svc/)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)
- [Claude Code Documentation](https://docs.claude.com/claude-code)

---

**Happy Coding!** 🦀✨

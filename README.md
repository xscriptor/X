
# X

## **Notice:** This repository is connected to the [Xscriptor-dev](https://github.com/xscriptordev/) repositories, to the brand consolidation.

### *Experimental space for system customization development*.

## Structure:

- [Basics](./basics/readme.md) *Includes post-installation scripts, applications, and documentation of installation things that I regularly use on daily-use systems.*
- [General](./general/README.md) *Includes specific customizations for application interfaces that I regularly use in my daily-use system.*
- x Automations to install de X environment on different distributions.

---
<details>

<summary>Tree</summary>

```tsx
X
x-> scripts related to xos 
├── basics
│   ├── linux
│   │   ├── desktop
│   │   │   └── gnome
│   │   │       └── openbar
│   │   │           └── configfiles
│   │   └── terminal
│   │       ├── alias
│   │       │   ├── github
│   │       │   └── navigation
│   │       ├── installations
│   │       │   ├── monitorization
│   │       │   └── qemu
│   │       └── scripts
│   │           └── github
│   └── windows
│       ├── postinstallation
│       ├── taskbarcustomization
│       │   └── xw11-taskbar
│       └── yasb
│           ├── config
│           └── preview
├── general
   ├── Jetbrains
   │   └── xscriptor-theme
   │       ├── download
   │       ├── gradle
   │       │   └── wrapper
   │       ├── preview
   │       └── src
   │           └── main
   │               └── resources
   │                   ├── META-INF
   │                   ├── colors
   │                   └── themes
   ├── custom-tools

```
</details>


- [zsh](./x/zsh/script.sh) *XBase for zsh*
```bash
curl -sLO https://raw.githubusercontent.com/xscriptor/X/main/x/zsh/script.sh && chmod +x script.sh && ./script.sh
```
# XDracula theme for Kitty

---

*My personal aspect for the Kitty Terminal based on Dracula theme*

### [XDracula](./kitty.conf)

## Installation Instructions

1. **Download the `kitty.conf` file**  
    Copy it to your configuration path:
    ```bash
    ~/.config/kitty/
    ```


2. **Use it as a theme include**  
    Create the theme directory if it doesn’t exist:
```bash
mkdir -p ~/.config/kitty/themes/XDracula
```
    Then include the theme in your main config file:

```bash
echo "include themes/XDracula/kitty.conf" >> ~/.config/kitty/kitty.conf
```
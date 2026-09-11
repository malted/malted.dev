Install


cargo build --release && printf '<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">\n<plist version="1.0"><dict><key>Label</key><string>dev.malted.maltedd</string><key>ProgramArguments</key><array><string>%s</string></array><key>KeepAlive</key><true/><key>RunAtLoad</key><true/><key>StandardOutPath</key><string>/tmp/maltedd.log</string><key>StandardErrorPath</key><string>/tmp/maltedd.err</string></dict></plist>\n' "$PWD/target/release/maltedd" > ~/Library/LaunchAgents/dev.malted.maltedd.plist && launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/dev.malted.maltedd.plist

---

Uninstall:
launchctl bootout gui/$(id -u)/dev.malted.maltedd; rm -f ~/Library/LaunchAgents/dev.malted.maltedd.plist /tmp/maltedd.log /tmp/maltedd.err

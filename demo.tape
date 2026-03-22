Output demo.gif

Set FontSize 14
Set Width 1200
Set Height 720
Set Padding 10
Set Theme "Dracula"

# Start ollamaMQ with multiple backends
Type "cargo run -- --ollama-urls http://localhost:11434,http://192.168.1.18:11434"
Sleep 500ms
Enter

# Wait for UI to stabilize and health checks to run
Sleep 3s

# Show help overlay
Type "?"
Sleep 4s
Type "?"

# Navigate through active users
Sleep 1s
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Type "k"
Sleep 300ms
Type "k"

# Demonstrate Boost and VIP features
Sleep 1s
Type "b"
Sleep 1.5s
Type "j"
Type "p"
Sleep 2s

# Switch to Blocked Items panel
Type "l"
Sleep 2s
Type "h"

# Stay a bit to show activity (run test_dispatcher.sh in another terminal now!)
Sleep 12s

# Exit
Type "q"

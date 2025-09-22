# Todo
- Spawn the player entity at the start position
- Add keyboard input movment
- Implement collision detections with walls
- Make it work on web

- Create headless Bevy app for server logic.
- Add Maze + player ECS components to server.
- Handle player inputs sent from clients.
- Update authoritative player positions.
- Broadcast state updates to clients.

- Choose transport: WebSockets (for browsers).
- Implement server WebSocket using Axum.
- Implement client WebSocket in WASM Bevy.
- Send input messages from client → server.
- Send state updates from server → client.
- Implement maze reset messages from server → client.

- Implement WebSocket client in Bevy WASM.
- Send movement input messages from client → server.
- Receive state updates from server → update client ECS.
- Render other players and maze correctly.
- Smooth movement (optional: client-side prediction/interpolation).

- Optimize rendering for large mazes (sprites → mesh/texture).
- Optimize network messages (binary, compressed, only send relevant data).
- Test with multiple browser clients.
- Fine-tune tick rate, maze reset timing, and input responsiveness.
- Optional: add visual effects, start/finish, leaderboard, or UI.